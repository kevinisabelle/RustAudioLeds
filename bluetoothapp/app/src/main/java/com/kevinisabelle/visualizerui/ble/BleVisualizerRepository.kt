package com.kevinisabelle.visualizerui.ble

import android.annotation.SuppressLint
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothDevice
import android.bluetooth.le.ScanCallback
import android.bluetooth.le.ScanSettings
import android.content.Context
import com.kevinisabelle.visualizerui.data.ParameterSpec
import kotlinx.coroutines.CancellationException
import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.withContext

/** ************************************************************
 * Repository responsible for all BLE operations used by the app
 * ************************************************************ */
class BleVisualizerRepository(
    private val context: Context,
) {
    private var currentDevice: BleVisualizerDevice? = null

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
            override fun onScanResult(callbackType: Int, result: android.bluetooth.le.ScanResult) {
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
        // Cancel any previous connection attempt.
        cancelConnect()

        // Connect using BleVisualizerDevice.
        // A new CoroutineScope is created here; alternatively, you may reuse a repository-level scope.
        currentDevice = BleVisualizerDevice.connect(context, device, CoroutineScope(Dispatchers.IO))

        // Wait until the device state is not Connecting.
        val finalState = currentDevice!!.state.first { it !is ConnectionState.Connecting }

        when (finalState) {
            is ConnectionState.Connected -> ConnectResult.Success(currentDevice!!)
            is ConnectionState.Failed -> ConnectResult.Error("Connection failed: ${finalState.reason}")
            is ConnectionState.Disconnected -> ConnectResult.Error("Disconnected")
            else -> ConnectResult.Error("Unknown connection outcome")
        }
    }

    /** Cancel current connection attempt or close existing connection. */
    @SuppressLint("MissingPermission")
    suspend fun cancelConnect() {
        currentDevice?.disconnect()
        currentDevice = null
    }

    /**
     * Reads the LED buffer from the device.
     */
    suspend fun readLedsBuffer(): ByteArray? {
        return try {
            currentDevice?.read(ParameterSpec.LedsBuffer)
        } catch (e: Exception) {
            null
        }
    }

    /**
     * Sets the running state of the visualizer.
     * @param running True to run the visualizer, false to pause.
     */
    suspend fun setRunning(running: Boolean) {

        // Do nothing for now
        // as the running state is not implemented in the device.

    }

    /**
     * Sets the gain/brightness of the visualizer.
     * @param gain The gain value (e.g., 0.0f to 1.0f).
     */
    suspend fun setGain(gain: Float) {
        try {
            currentDevice?.write(ParameterSpec.Gain, gain)
        } catch (e: Exception) {
            // Handle error if necessary.
        }
    }

    suspend fun getLedColors() : List<Int> {
        println("Fetching LED colors from device...")
        val result1 = currentDevice?.read(ParameterSpec.LedsBuffer)
        val result2 = currentDevice?.read(ParameterSpec.LedsBuffer2)

        // Assemble both buffers if they are not null
        val result = if (result1 != null && result2 != null) {
            result1 + result2
        } else if (result1 != null) {
            result1
        } else if (result2 != null) {
            result2
        } else {
            null
        }

        // Take everything in the buffer until the first null byte
        val ledBufferSize = ParameterSpec.LedsBuffer.SIZE + ParameterSpec.LedsBuffer2.SIZE
        val ledBuffer = result?.take(ledBufferSize) ?: return emptyList()

        // println("LED buffer content (HEX): ${ledBuffer?.joinToString(" ") { String.format("%02X", it) }}")

        if (ledBuffer == null) {
            println("Failed to fetch LED colors: Device not connected or read failed.")
            return emptyList()
        }

        if (ledBuffer.size % 3 != 0) {
            println("Error: LED buffer size is not a multiple of 3, size=${result.size}")
            return emptyList()
        }

        // Convert ByteArray to List<Int> representing GRB colors
        val ledColors = ledBuffer.chunked(3) { chunk ->
            if (chunk.size == 3) {
                // Convert RGB888 to Int (ARGB format)
                val g = chunk[0].toInt() and 0xFF
                val r = chunk[1].toInt() and 0xFF
                val b = chunk[2].toInt() and 0xFF
                (0xFF shl 24) or (r shl 16) or (g shl 8) or b // ARGB format
            } else {
                println("Warning: Incomplete color chunk found, size=${chunk.size}")
                0 // Default to black if chunk is incomplete
            }
        }

        // println("LED colors fetched: $ledColors")
        return ledColors
    }

}