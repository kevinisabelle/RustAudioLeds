package com.kevinisabelle.visualizerui.ble

import android.Manifest
import android.annotation.SuppressLint
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothGatt
import android.bluetooth.BluetoothGattCallback
import android.bluetooth.BluetoothProfile
import android.bluetooth.le.ScanCallback
import android.bluetooth.le.ScanSettings
import android.content.Context
import androidx.annotation.RequiresPermission
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.emptyFlow
import kotlinx.coroutines.suspendCancellableCoroutine
import kotlinx.coroutines.withContext
import kotlin.coroutines.resume
import android.bluetooth.le.ScanResult as SysScanResult

/** ************************************************************
 * Repository responsible for all BLE operations used by the app
 * ************************************************************ */
class BleVisualizerRepository(
    private val context: Context,
) {
    private var currentDevice: BleVisualizerDevice? = null
    private var ongoingGatt: BluetoothGatt? = null

    /** One‑shot scan returning after [timeoutMs]. */
    suspend fun scanOnce(timeoutMs: Long = 5_000): ScanResult = withContext(Dispatchers.IO) {
        val adapter = BluetoothAdapter.getDefaultAdapter()
            ?: return@withContext ScanResult.Error("Bluetooth unavailable")
        if (!adapter.isEnabled) {
            return@withContext ScanResult.Error(
                message = "Bluetooth is off",
                actionLabel = "Turn on",
                recoveryAction = { retry ->
                    // context.startActivity(Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE))
                    retry()
                }
            )
        }

        val scanResults = mutableMapOf<String, ScannedDevice>()
        val scanner = adapter.bluetoothLeScanner
        val callback = object : ScanCallback() {
            @SuppressLint("MissingPermission")
            override fun onScanResult(callbackType: Int, result: SysScanResult) {
                val device = result.device
                val key = device.address
                val name = device.name ?: "Unknown"
                scanResults[key] = ScannedDevice(device, name, result.rssi)
            }
        }

        return@withContext try {
            scanner.startScan(null, ScanSettings.Builder().build(), callback)
            delay(timeoutMs)
            scanner.stopScan(callback)
            ScanResult.Success(scanResults.values.toList())
        } catch (sec: SecurityException) {
            ScanResult.Error("Permissions missing or denied")
        } catch (e: CancellationException) {
            throw e // coroutine cancelled
        } catch (e: Exception) {
            ScanResult.Error(e.localizedMessage ?: "Scan failed")
        }
    }

    /** Connect to [device] then discover services; returns on success or error. */
    @SuppressLint("MissingPermission")
    suspend fun connectAndDiscover(device: BluetoothDevice): ConnectResult = withContext(Dispatchers.IO) {
        // cancel any previous attempt
        cancelConnect()

        suspendCancellableCoroutine { cont ->
            val callback = object : BluetoothGattCallback() {
                override fun onConnectionStateChange(gatt: BluetoothGatt, status: Int, newState: Int) {
                    if (status != BluetoothGatt.GATT_SUCCESS) {
                        cleanUp(gatt)
                        if (cont.isActive) cont.resume(ConnectResult.Error("Connection error ($status)"))
                        return
                    }
                    if (newState == BluetoothProfile.STATE_CONNECTED) {
                        gatt.discoverServices()
                    } else if (newState == BluetoothProfile.STATE_DISCONNECTED) {
                        cleanUp(gatt)
                        if (cont.isActive) cont.resume(ConnectResult.Error("Disconnected"))
                    }
                }

                override fun onServicesDiscovered(gatt: BluetoothGatt, status: Int) {
                    if (status == BluetoothGatt.GATT_SUCCESS) {
                        ongoingGatt = gatt
                        if (cont.isActive) cont.resume(ConnectResult.Success(gatt))
                    } else {
                        cleanUp(gatt)
                        if (cont.isActive) cont.resume(ConnectResult.Error("Service discovery failed ($status)"))
                    }
                }
            }

            val gatt = device.connectGatt(context, false, callback, BluetoothDevice.TRANSPORT_LE)
            ongoingGatt = gatt

            cont.invokeOnCancellation { throwable ->
                // cancel triggered by caller
                try {
                    cleanUp(gatt)
                } finally {
                    if (!cont.isCompleted) cont.resume(ConnectResult.Cancelled)
                }
            }
        }
    }

    /** Cancel current connection attempt or close existing GATT. */
    @SuppressLint("MissingPermission")
    fun cancelConnect() {
        ongoingGatt?.let { cleanUp(it) }
        ongoingGatt = null
    }

    /** Helper to close/refresh gatt safely. */
    @RequiresPermission(Manifest.permission.BLUETOOTH_CONNECT)
    private fun cleanUp(gatt: BluetoothGatt) {
        try {
            gatt.disconnect()
        } catch (_: Exception) {}
        try {
            gatt.close()
        } catch (_: Exception) {}
    }

    /**
     * Flow that emits lists of RGB integer values for the LEDs.
     * Each Int should represent a packed RGB color.
     */
    fun ledBufferFlow(): Flow<List<Int>> {
        // Placeholder implementation: returns an empty flow.
        // Replace with actual BLE characteristic reading logic.
        return emptyFlow()
    }

    /**
     * Sets the running state of the visualizer.
     * @param running True to run the visualizer, false to pause.
     */
    suspend fun setRunning(running: Boolean) {
        // Placeholder implementation:
        // Add logic to send the running state to the BLE device.
        // For example, writing to a BLE characteristic.
        withContext(Dispatchers.IO) {
            // ongoingGatt?.let { gatt ->
            //    val service = gatt.getService(YOUR_SERVICE_UUID)
            //    val characteristic = service?.getCharacteristic(YOUR_RUNNING_CHARACTERISTIC_UUID)
            //    characteristic?.let {
            //        it.value = byteArrayOf(if (running) 1.toByte() else 0.toByte())
            //        gatt.writeCharacteristic(it)
            //    }
            // }
        }
    }

    /**
     * Sets the gain/brightness of the visualizer.
     * @param gain The gain value (e.g., 0.0f to 1.0f).
     */
    suspend fun setGain(gain: Float) {
        // Placeholder implementation:
        // Add logic to send the gain value to the BLE device.
        // For example, writing to a BLE characteristic.
        withContext(Dispatchers.IO) {
            // ongoingGatt?.let { gatt ->
            //    val service = gatt.getService(YOUR_SERVICE_UUID)
            //    val characteristic = service?.getCharacteristic(YOUR_GAIN_CHARACTERISTIC_UUID)
            //    characteristic?.let {
            //        // Convert float to appropriate byte array format for your device
            //        // val gainBytes = ...
            //        // it.value = gainBytes
            //        // gatt.writeCharacteristic(it)
            //    }
            // }
        }
    }

}
