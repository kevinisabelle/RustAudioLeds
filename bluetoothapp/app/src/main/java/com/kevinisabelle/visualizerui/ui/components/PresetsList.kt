package com.kevinisabelle.visualizerui.ui.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import com.kevinisabelle.visualizerui.data.Preset

@Composable
fun PresetsList(
    presets: List<Preset>,
    onPresetSelected: (Preset) -> Unit,
    onPresetDeleted: (Preset) -> Unit = {},
    onRefreshClick: () -> Unit = {}
) {

    val scrollState = rememberScrollState()
    Column(
        modifier = Modifier.padding(8.dp).verticalScroll(scrollState),
        verticalArrangement = Arrangement.spacedBy(8.dp)
    ) {
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.Center
        )
        {
            IconButton(
                onClick = { onRefreshClick() },
                modifier = Modifier.padding(start = 8.dp)
            ) {
                Icon(
                    imageVector = Icons.Default.Refresh,
                    contentDescription = "Reload presets"
                )
            }
            Text(
                text = "Presets",
                style = MaterialTheme.typography.titleLarge,
                textAlign = TextAlign.Center,
                modifier = Modifier.weight(1f)
            )
        }

        for (preset in presets) {
            PresetCard(
                preset = preset,
                onActivateClick = { onPresetSelected(preset) },
                onDeleteClick = { onPresetDeleted(preset) },
                modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp)
            )
        }
    }
}


