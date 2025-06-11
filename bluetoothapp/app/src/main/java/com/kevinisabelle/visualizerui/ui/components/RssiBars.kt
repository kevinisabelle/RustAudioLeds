package com.kevinisabelle.visualizerui.ui.components

import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.SignalWifi0Bar
import androidx.compose.material.icons.filled.SignalWifi4Bar
import androidx.compose.material.icons.filled.SignalWifiBad
import androidx.compose.material.icons.filled.SignalWifiOff
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

@Composable
fun RssiBars(rssi: Int) {

    // This could be a series of colored bars or any other visual representation
    val color = when {
        rssi >= -50 -> MaterialTheme.colorScheme.primary // Excellent signal
        rssi >= -70 -> MaterialTheme.colorScheme.secondary // Good signal
        rssi >= -85 -> MaterialTheme.colorScheme.tertiary // Fair signal
        else -> MaterialTheme.colorScheme.error // Poor signal
    }

    val icon = when {
        rssi >= -50 -> Icons.Default.SignalWifi4Bar // Excellent signal
        rssi >= -70 -> Icons.Default.SignalWifi0Bar // Good signal
        rssi >= -85 -> Icons.Default.SignalWifiBad // Fair signal
        else -> Icons.Default.SignalWifiOff // Poor signal
    }

    Icon(
        imageVector = icon,
        contentDescription = "RSSI Indicator",
        tint = color,
        modifier = Modifier.padding(4.dp)
    )

    Text(
        text = "$rssi dBm",
        modifier = Modifier.padding(4.dp),
        color = color
    )
}
