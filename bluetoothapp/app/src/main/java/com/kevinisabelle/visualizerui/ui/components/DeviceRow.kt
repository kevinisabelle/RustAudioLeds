package com.kevinisabelle.visualizerui.ui.components

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Card
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.kevinisabelle.visualizerui.ble.ScannedDevice

@Composable
fun DeviceRow(
    device: ScannedDevice,
    onConnect: (ScannedDevice) -> Unit
) = Card(
    onClick = { onConnect(device) },
    modifier = Modifier
        .fillMaxWidth()
        .padding(horizontal = 16.dp, vertical = 4.dp)
) {
    Row(
        Modifier
            .fillMaxWidth()
            .padding(12.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        Column(Modifier.weight(1f)) {
            Text(device.displayName, style = MaterialTheme.typography.titleMedium)
            Text(device.device.address, style = MaterialTheme.typography.bodySmall)
        }
        RssiBars(device.rssi)
    }
}

