package com.kevinisabelle.visualizerui.ble

import android.annotation.SuppressLint
import android.bluetooth.*
import android.content.Context
import com.kevinisabelle.visualizerui.data.ParameterSpec
import kotlinx.coroutines.*
import kotlinx.coroutines.channels.awaitClose
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock
import java.util.UUID

class BleVisualizerDevice private constructor(
    private val context: Context,
    private val device: BluetoothDevice,
    private val scope: CoroutineScope
) {
    /* ---------- Public API ---------- */

    val state: StateFlow<ConnectionState> get() = _state
    @SuppressLint("MissingPermission")
    suspend fun <T : Any> read(spec: ParameterSpec<T>): T = doGattIo {
        val ch = ch(spec)
        gatt.readCharacteristic(ch)
        eventFlow.first { it is GattEvent.Result && it.type == ResultType.Read && it.uuid == spec.uuid }
        decode(spec, ch.value)
    }

    @SuppressLint("MissingPermission")
    suspend fun <T : Any> write(spec: ParameterSpec<T>, value: T) {
        doGattIo {
            val ch = ch(spec)
            ch.value = encode(spec, value)
            ch.writeType = BluetoothGattCharacteristic.WRITE_TYPE_NO_RESPONSE
            gatt.writeCharacteristic(ch)
            eventFlow.first { it is GattEvent.Result &&
                    it.type == ResultType.Write &&
                    it.uuid == spec.uuid }
            Unit
        }
    }

    @SuppressLint("MissingPermission")
    suspend fun disconnect() = withContext(Dispatchers.IO) {
        gatt.close()
        _state.value = ConnectionState.Disconnected
    }

    /* ---------- Setup / factory ---------- */

    companion object {
        val SERVICE_UUID: UUID = UUID.fromString("3E0E0000-7C7A-47B0-9FD5-1FC3044C3E63")

        /**
         * Initiates the connection and emits state changes as they arrive.
         */
        fun connect(
            ctx: Context,
            device: BluetoothDevice,
            parentScope: CoroutineScope
        ): BleVisualizerDevice {
            val dev = BleVisualizerDevice(ctx, device, parentScope)
            dev.initGatt()
            return dev
        }
    }

    /* ---------- Internals ---------- */

    private sealed interface GattEvent {
        object ServiceReady : GattEvent
        data class Result(
            val type: ResultType,
            val uuid: UUID,
            val status: Int
        ) : GattEvent
    }

    /* ---------- Tiny DTO ---------- */
    private data class GattResult(
        val type: ResultType,
        val uuid: UUID,
        val status: Int
    )

    private enum class ResultType { Read, Write }

    private lateinit var gatt: BluetoothGatt
    private val _state = MutableStateFlow<ConnectionState>(ConnectionState.Connecting)

    /** Serialized I/O; avoids overlapping read/write ops (BLE spec limitation). */
    private val ioMutex = Mutex()

    /** Channel of callback events → Flow for suspend until-complete semantics. */
    @SuppressLint("MissingPermission")
    private val eventFlow: SharedFlow<GattEvent> = callbackFlow {
        val cb = object : BluetoothGattCallback() {

            override fun onConnectionStateChange(gatt: BluetoothGatt, status: Int, newState: Int) {
                when (newState) {
                    BluetoothProfile.STATE_CONNECTED -> {
                        gatt.discoverServices()
                        _state.value = ConnectionState.Connected(UUID.fromString(device.address))
                    }
                    BluetoothProfile.STATE_DISCONNECTED -> {
                        _state.value = ConnectionState.Disconnected
                        trySend(GattEvent.ServiceReady) // Emit Unit to unblock init.
                    }
                    else -> {
                        _state.value = ConnectionState.Failed("Unknown state: $newState")
                    }
                }
            }

            override fun onServicesDiscovered(gatt: BluetoothGatt, status: Int) {
                trySend(GattEvent.ServiceReady)
            }

            override fun onCharacteristicRead(gatt: BluetoothGatt,
                                              characteristic: BluetoothGattCharacteristic,
                                              status: Int) {
                trySend(
                    GattEvent.Result(ResultType.Read, characteristic.uuid, status)
                )
            }

            override fun onCharacteristicWrite(gatt: BluetoothGatt,
                                               characteristic: BluetoothGattCharacteristic,
                                               status: Int) {
                trySend(
                    GattEvent.Result(ResultType.Write, characteristic.uuid, status)
                )
            }
        }

        gatt = device.connectGatt(context, false, cb, BluetoothDevice.TRANSPORT_LE)
        awaitClose { gatt.close() }
    }.shareIn(scope, SharingStarted.Eagerly, replay = 0)

    private fun initGatt() {
        // gatt is launched via callbackFlow above
    }

    /** Helper: ensure GATT tree is ready, then execute critical section. */
    private suspend fun <R> doGattIo(block: suspend IoContext.() -> R): R =
        withContext(Dispatchers.IO) {
            ioMutex.withLock {
                // Wait until services are discovered (first Unit emission):
                eventFlow.first { it is GattEvent.ServiceReady }
                block(IoContext())
            }
        }

    /** Narrow accessor for code-completion clarity inside the mutex-protected block. */
    private inner class IoContext {
        fun ch(spec: ParameterSpec<*>): BluetoothGattCharacteristic =
            gatt.getService(SERVICE_UUID)
                ?.getCharacteristic(spec.uuid)
                ?: error("Characteristic ${spec.uuid} not found!")
    }
}