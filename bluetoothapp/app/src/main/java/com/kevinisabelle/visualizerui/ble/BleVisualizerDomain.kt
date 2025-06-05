package com.kevinisabelle.visualizerui.ble

import android.annotation.SuppressLint
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothDevice
import android.bluetooth.le.ScanCallback
import android.bluetooth.le.ScanSettings
import android.content.Context
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.withContext
import android.bluetooth.le.ScanResult as SysScanResult

/** *******************************************
 * Simple domain layer objects for BLE scanning
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
        val recoveryAction: (suspend (() -> Unit) -> Unit)? = null, // call with retry lambda
    ) : ScanResult
}

/** UI mode/state for the ScanScreen. */
sealed interface ScanUi {
    data object Scanning : ScanUi
    data object DeviceList : ScanUi
    data object Error : ScanUi
}

/** ************************************************************
 * Repository responsible for all BLE operations used by the app
 * ************************************************************ */
class BleVisualizerRepository(
    private val context: Context,
) {
    private val _scanState = MutableStateFlow<ScanResult?>(null)
    val scanState = _scanState.asStateFlow()

    /** One‑shot scan returning after [timeoutMs]. */
    suspend fun scanOnce(timeoutMs: Long = 5_000): ScanResult = withContext(Dispatchers.IO) {
        val adapter = BluetoothAdapter.getDefaultAdapter()
            ?: return@withContext ScanResult.Error("Bluetooth unavailable")
        if (!adapter.isEnabled) {
            return@withContext ScanResult.Error(
                message = "Bluetooth is off",
                actionLabel = "Turn on",
                recoveryAction = { retry ->
                    // Fire an enable intent then retry once the caller decides
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
}
