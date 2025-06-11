package com.kevinisabelle.visualizerui.ui.components

import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.kevinisabelle.visualizerui.ble.ScannedDevice

@Composable
fun DeviceList(
    devices: List<ScannedDevice>, // Replace Any with your actual device type
    onConnect: (ScannedDevice) -> Unit, // Replace Any with your actual device type
    modifier: Modifier = Modifier
) {
    LazyColumn(modifier = modifier) {
        items(devices) { device ->
            DeviceRow(
                device = device,
                onConnect = onConnect
            )
        }
    }
}

@Preview
@Composable
fun DeviceListPreview() {
    DeviceList(
        devices = run {
            val bluetoothAdapter: android.bluetooth.BluetoothAdapter? = try {
                android.bluetooth.BluetoothAdapter.getDefaultAdapter()
            } catch (e: Throwable) {
                null // Catch any unexpected errors during getDefaultAdapter
            }

            if (bluetoothAdapter != null) {
                try {
                    val address1 = "00:11:22:33:44:55"
                    val address2 = "AA:BB:CC:DD:EE:FF" // Using a different valid MAC address

                    val btDevice1 = bluetoothAdapter.getRemoteDevice(address1)
                    val btDevice2 = bluetoothAdapter.getRemoteDevice(address2)

                    listOf(
                        ScannedDevice(btDevice1, "LedVisualizer", -50),
                        ScannedDevice(btDevice2, "Some Random device", -60),
                        ScannedDevice(btDevice2, "Some Random device 2", -90)
                    )
                } catch (e: SecurityException) {
                    // Handle SecurityException (e.g., on API 31+ if BLUETOOTH_CONNECT is missing)
                    emptyList<ScannedDevice>() // Fallback to an empty list
                } catch (e: IllegalArgumentException) {
                    // Handle IllegalArgumentException (if MAC address format is invalid)
                    emptyList<ScannedDevice>() // Fallback to an empty list
                }
            } else {
                // BluetoothAdapter is not available
                emptyList<ScannedDevice>() // Fallback to an empty list
            }
        },
        onConnect = {}
    )
}