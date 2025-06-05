package com.kevinisabelle.visualizerui.ui.components

import androidx.compose.foundation.layout.padding
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp

@Composable
fun RssiBars(rssi: Int) {

    // Placeholder for RSSI visualization logic
    // This could be a series of colored bars or any other visual representation
    Text(
        text = "RSSI: $rssi dBm",
        style = MaterialTheme.typography.bodySmall,
        modifier = Modifier.padding(start = 8.dp)
    )
}
