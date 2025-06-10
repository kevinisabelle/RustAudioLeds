package com.kevinisabelle.visualizerui.ui.components

import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.kevinisabelle.visualizerui.ble.ScannedDevice

@Composable
fun DeviceList(
    devices: List<ScannedDevice>, // Replace Any with your actual device type
    onConnect: (ScannedDevice) -> Unit, // Replace Any with your actual device type
    modifier: Modifier = Modifier
) {
    LazyColumn(modifier = modifier) {
        items(devices) { device ->
            Text(
                text = device.toString(), // Replace with actual device property e.g. device.name
                modifier = Modifier
                    .fillMaxWidth()
                    .clickable { onConnect(device) }
                    .padding(16.dp)
            )
        }
    }
}