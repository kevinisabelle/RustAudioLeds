package com.kevinisabelle.visualizerui.ble

import android.Manifest
import android.annotation.SuppressLint
import android.bluetooth.*
import android.bluetooth.le.ScanCallback
import android.bluetooth.le.ScanSettings
import android.content.Context
import androidx.annotation.RequiresPermission
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.*
import kotlinx.coroutines.suspendCancellableCoroutine
import kotlinx.coroutines.withContext
import kotlin.coroutines.resume
import kotlin.coroutines.resumeWithException
import android.bluetooth.le.ScanResult as SysScanResult

/** *******************************************
 * Simple domain layer objects for BLE scanning & connecting
 * ******************************************* */

/** Wrapper for one BLE advertisement. */
data class ScannedDevice(
    val device: BluetoothDevice,
    val displayName: String,
    val rssi: Int,
)

/** Results of a single scan attempt. */
sealed interface ScanResult {
    data class Success(val devices: List<ScannedDevice>) : ScanResult
    data class Error(
        val message: String,
        val actionLabel: String = "",
        val recoveryAction: (suspend ((/* retry */) -> Unit) -> Unit)? = null,
    ) : ScanResult
}

/** UI mode/state for the ScanScreen. */
sealed interface ScanUi {
    data object Scanning : ScanUi
    data object DeviceList : ScanUi
    data object Error : ScanUi
}

/** Results of a connect‑&‑discover sequence. */
sealed interface ConnectResult {
    data class Success(val gatt: BluetoothGatt) : ConnectResult
    data class Error(val message: String) : ConnectResult
    data object Cancelled : ConnectResult
}

/** ************************************************************
 * Repository responsible for all BLE operations used by the app
 * ************************************************************ */
class BleVisualizerRepository(
    private val context: Context,
) {
    /* ---------- Scan state ---------- */
    private val _scanState = MutableStateFlow<ScanResult?>(null)
    val scanState = _scanState.asStateFlow()

    /* ---------- Connection state ---------- */
    private val _connectState = MutableStateFlow<ConnectResult?>(null)
    val connectState = _connectState.asStateFlow()

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
}
