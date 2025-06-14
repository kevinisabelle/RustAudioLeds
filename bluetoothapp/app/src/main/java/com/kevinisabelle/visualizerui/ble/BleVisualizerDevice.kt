package com.kevinisabelle.visualizerui.ble

import android.annotation.SuppressLint
import android.bluetooth.*
import android.content.Context
import com.kevinisabelle.visualizerui.data.ParameterSpec
import kotlinx.coroutines.*
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.flow.*
import android.util.Log
import java.util.UUID

class BleVisualizerDevice private constructor(
    private val context: Context,
    private val device: BluetoothDevice,
    private val scope: CoroutineScope
) {
    /* ---------- Public API ---------- */

    val LOG_TAG = "BleVisualizerDevice"
    val state: StateFlow<ConnectionState> get() = _state

    // Field to signal service discovery completion
    private var servicesDiscoveredCompleter = CompletableDeferred<Unit>()

    @SuppressLint("MissingPermission")
    suspend fun <T : Any> read(spec: ParameterSpec<T>): T? = doGattIo {
        Log.d(LOG_TAG, "Attempting to read characteristic ${spec.uuid}...")
        try {
            val ch = ch(spec)
            gatt.readCharacteristic(ch)
            val result = eventFlow.onEach { event ->
                Log.d(LOG_TAG,"Received event: $event")
            }.first {
                it is GattEvent.Result &&
                        it.type == ResultType.Read &&
                        it.uuid == spec.uuid
            } as GattEvent.Result

            if (result.status != BluetoothGatt.GATT_SUCCESS) {
                throw IllegalStateException(
                    "Failed to read characteristic ${spec.uuid}. Status: ${result.status}"
                )
            }
            // decode needs to be inside the try block if ch is declared inside
            Log.d(LOG_TAG, "Read successful for characteristic ${spec.uuid}.")
            return@doGattIo decode(spec, ch.value ?: error("Characteristic ${spec.uuid} has no value!"))
        }
        catch (e: IllegalStateException) {
            // If the read operation was cancelled or failed, or characteristic not found.
            Log.e(LOG_TAG, "Error reading characteristic ${spec.uuid}: ${e.message}")
            return@doGattIo null
        }
    }

    @SuppressLint("MissingPermission")
    suspend fun <T : Any> write(spec: ParameterSpec<T>, value: T) {
        doGattIo {
            try {
                Log.d(LOG_TAG, "Attempting to write to characteristic ${spec.uuid} with value: $value (no response)...")
            val ch = ch(spec)
            ch.value = encode(spec, value)
            ch.writeType = BluetoothGattCharacteristic.WRITE_TYPE_NO_RESPONSE

            if (!gatt.writeCharacteristic(ch)) {
                // This means the write operation could not even be initiated.
                Log.e(LOG_TAG, "Failed to initiate write operation for characteristic ${spec.uuid}")
                    return@doGattIo
            }

            // For WRITE_TYPE_NO_RESPONSE, the onCharacteristicWrite callback is not triggered.
            // If gatt.writeCharacteristic(ch) returns true, the operation was successfully initiated.
            // There will be no further confirmation from the BLE stack for this type of write.
            Log.d(LOG_TAG, "Write (no response) to characteristic ${spec.uuid} initiated successfully.")
            } catch (e: Exception) {
                Log.e(LOG_TAG, "Error writing to characteristic ${spec.uuid}: ${e.message}")
                // throw IllegalStateException("Failed to write to characteristic ${spec.uuid}", e)
            }
        }
    }

    @SuppressLint("MissingPermission")
    suspend fun disconnect() = withContext(Dispatchers.IO) {
        if (::gatt.isInitialized) {
            gatt.disconnect() // Triggers onConnectionStateChange -> STATE_DISCONNECTED
            gatt.close()      // Release resources. awaitClose will also attempt this.
        } else {
            // If gatt not initialized, ensure state and completer are correctly set.
            _state.value = ConnectionState.Disconnected
            if (!servicesDiscoveredCompleter.isCancelled && !servicesDiscoveredCompleter.isCompleted) {
                servicesDiscoveredCompleter.cancel(CancellationException("Disconnected (GATT not initialized)"))
            }
            if (servicesDiscoveredCompleter.isCompleted || servicesDiscoveredCompleter.isCancelled) {
                servicesDiscoveredCompleter = CompletableDeferred() // Reset
            }
        }
    }

    /* ---------- Setup / factory ---------- */

    companion object {
        val SERVICE_UUID: UUID = UUID.fromString("3E0E0000-7C7A-47B0-9FD5-1FC3044C3E63")

        fun connect(
            ctx: Context,
            device: BluetoothDevice,
            parentScope: CoroutineScope
        ): BleVisualizerDevice {
            val dev = BleVisualizerDevice(ctx, device, parentScope)
            dev.initGatt() // This starts the connection process via callbackFlow
            return dev
        }
    }

    /* ---------- Internals ---------- */

    // GattEvent no longer needs ServiceReady
    private sealed interface GattEvent {
        data class Result(
            val type: ResultType,
            val uuid: UUID,
            val status: Int
        ) : GattEvent
    }

    private enum class ResultType { Read, Write }

    private lateinit var gatt: BluetoothGatt
    private val _state = MutableStateFlow<ConnectionState>(ConnectionState.Connecting)

    @SuppressLint("MissingPermission")
    private val eventFlow: SharedFlow<GattEvent> = callbackFlow<GattEvent> {
        val cb = object : BluetoothGattCallback() {
            override fun onConnectionStateChange(gatt: BluetoothGatt, status: Int, newState: Int) {
                scope.launch {
                    when (newState) {
                        BluetoothProfile.STATE_CONNECTED -> {
                            _state.value = ConnectionState.Connecting // Still connecting until services are discovered
                            if (servicesDiscoveredCompleter.isCompleted || servicesDiscoveredCompleter.isCancelled) {
                                servicesDiscoveredCompleter = CompletableDeferred()
                            }
                            if (!gatt.discoverServices()) {
                                _state.value = ConnectionState.Failed("Failed to initiate service discovery.")
                                servicesDiscoveredCompleter.completeExceptionally(IllegalStateException("Failed to initiate service discovery."))
                                // Consider closing gatt here
                            }
                        }
                        BluetoothProfile.STATE_DISCONNECTED -> {
                            _state.value = ConnectionState.Disconnected
                            servicesDiscoveredCompleter.cancel(CancellationException("Device disconnected."))
                            servicesDiscoveredCompleter = CompletableDeferred() // Reset for potential future connections
                        }
                        BluetoothProfile.STATE_CONNECTING -> {
                            _state.value = ConnectionState.Connecting
                            if (servicesDiscoveredCompleter.isCompleted || servicesDiscoveredCompleter.isCancelled) {
                                servicesDiscoveredCompleter = CompletableDeferred()
                            }
                        }
                        else -> {
                            _state.value = ConnectionState.Failed("Connection failed or lost. Status: $status, NewState: $newState")
                            servicesDiscoveredCompleter.cancel(CancellationException("Connection failed or lost."))
                            servicesDiscoveredCompleter = CompletableDeferred() // Reset
                        }
                    }
                }
            }

            override fun onServicesDiscovered(gatt: BluetoothGatt, status: Int) {
                scope.launch {
                    if (status == BluetoothGatt.GATT_SUCCESS) {
                        _state.value = ConnectionState.Connected(device.address) // Fully connected now
                        servicesDiscoveredCompleter.complete(Unit)
                        gatt.requestMtu(1024) // Request a larger MTU if needed
                    } else {
                        _state.value = ConnectionState.Failed("Service discovery failed. Status: $status")
                        servicesDiscoveredCompleter.completeExceptionally(IllegalStateException("Service discovery failed. Status: $status"))
                        // Consider gatt.disconnect() / gatt.close() here
                    }
                }
            }

            override fun onMtuChanged(gatt: BluetoothGatt, mtu: Int, status: Int) {
                super.onMtuChanged(gatt, mtu, status)
                if (status == BluetoothGatt.GATT_SUCCESS) {
                    Log.d(LOG_TAG, "MTU successfully changed to: $mtu. Max data per packet for read: ${mtu - 1}")
                } else {
                    Log.e(LOG_TAG, "MTU change request failed. Status: $status. Current MTU likely remains default or previous.")
                }
            }

            @Deprecated("Use onCharacteristicRead(BluetoothGatt, BluetoothGattCharacteristic, byte[], int) for API 33+")
            override fun onCharacteristicRead(gatt: BluetoothGatt,
                                              characteristic: BluetoothGattCharacteristic,
                                              status: Int) {
                // Log the size of the data received by the callback
                val data = characteristic.value
                Log.d(LOG_TAG,"onCharacteristicRead: UUID=${characteristic.uuid}, Status=$status, Received Size=${data?.size ?: 0}")
                trySend(GattEvent.Result(ResultType.Read, characteristic.uuid, status))
            }
        }

        this@BleVisualizerDevice.gatt = device.connectGatt(context, false, cb, BluetoothDevice.TRANSPORT_LE)
        awaitClose {
            if (this@BleVisualizerDevice::gatt.isInitialized) {
            this@BleVisualizerDevice.gatt.close()
        }
        }
    }.shareIn(scope, SharingStarted.Eagerly, replay = 0)

    private fun initGatt() {
        // gatt initialization and connection attempt is started by the callbackFlow producer block
    }

    private suspend fun <R> doGattIo(block: suspend IoContext.() -> R): R =
        withContext(Dispatchers.IO) {
            try {
                // Wait for services to be discovered.
                servicesDiscoveredCompleter.await()
            } catch (e: CancellationException) { // Catch if await is cancelled (e.g. by disconnect)
                throw IllegalStateException("Operation cancelled: Device disconnected or service discovery incomplete.", e)
            } catch (e: Exception) { // Catch if completer completed exceptionally (e.g. discovery failed)
                throw IllegalStateException("Service discovery failed or services not available.", e)
            }

            // After servicesDiscoveredCompleter.await() returns, _state.value should be Connected.
            if (state.value !is ConnectionState.Connected) {
                throw IllegalStateException("Services ready, but device not in Connected state. Current state: ${state.value}")
            }

            block(IoContext())
        }

    private inner class IoContext {
        fun ch(spec: ParameterSpec<*>): BluetoothGattCharacteristic {
            val characteristic = gatt.getService(SERVICE_UUID)?.getCharacteristic(spec.uuid)

            if (characteristic != null) {
                return characteristic
            }

            // If not found, search all services
            for (service in gatt.services) {
                service.getCharacteristic(spec.uuid)?.let {
                    Log.v(LOG_TAG, "Found characteristic ${spec.uuid} in unexpected service: ${service.uuid}")
                    return it
                }
            }

            // Log all available characteristics to help debugging
            Log.v(LOG_TAG, "Characteristic ${spec.uuid} not found. Available characteristics:")
            gatt.services.forEach { service ->
                Log.v(LOG_TAG, "Service: ${service.uuid}")
                service.characteristics.forEach { char ->
                    Log.v(LOG_TAG, "  - ${char.uuid}")
                }
            }

            Log.e(LOG_TAG, "Characteristic ${spec.uuid} not found in any service!")
            error("Characteristic ${spec.uuid} not found!")
        }
    }
}