package com.kevinisabelle.visualizerui.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.PlayCircle
import androidx.compose.material.icons.filled.Star
import androidx.compose.material3.Card
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.IconButtonColors
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.TextUnit
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.em
import com.kevinisabelle.visualizerui.data.AnimationMode
import com.kevinisabelle.visualizerui.data.DisplayMode
import com.kevinisabelle.visualizerui.data.Preset
import com.kevinisabelle.visualizerui.data.Rgb888

@Composable
fun PresetCard(
    preset: Preset?,
    currentPresetIndex: UByte,
    onActivateClick: () -> Unit,
    onDeleteClick: () -> Unit,
    modifier: Modifier = Modifier
) {
    Card(
        modifier = modifier
    ) {
        Column {

            if (preset == null) {
                Text(
                    text = "Slot is empty",
                    modifier = Modifier.fillMaxWidth().padding(16.dp),
                )
                return@Column
            }

            Row(
                modifier = Modifier.padding(horizontal = 12.dp).fillMaxWidth(),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.SpaceBetween,
            ) {
                if (preset.index == currentPresetIndex) {
                    Icon(
                        imageVector = Icons.Default.Star,
                        contentDescription = "Default Preset",
                        tint = Color.Yellow
                    )
                }
                Text(text = "${preset.index} - ${preset.name}", style = MaterialTheme.typography.titleMedium)

                Row()
                {

                    IconButton(
                        onClick = onDeleteClick
                    ) {
                        Icon(
                            imageVector = Icons.Default.Delete,
                            contentDescription = "Delete Preset",
                            tint = MaterialTheme.colorScheme.error
                        )
                    }
                    IconButton(
                        onClick = onActivateClick,
                    ) {
                        Icon(
                            imageVector = Icons.Default.PlayCircle,
                            contentDescription = "Activate Preset",
                        )
                    }
                }
            }
            Row(
                modifier = Modifier.padding(vertical = 0.dp).fillMaxWidth().background(Color.Black),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.SpaceEvenly
            ) {
                // Display colors
                Text(text = preset.color1.toHex(), color = preset.color1.toStdColor(), style = MaterialTheme.typography.titleMedium, textAlign = TextAlign.Center, fontWeight = FontWeight.Bold)
                Text(text = preset.color2.toHex(), color = preset.color2.toStdColor(), style = MaterialTheme.typography.titleMedium, textAlign = TextAlign.Center, fontWeight = FontWeight.Bold)
                Text(text = preset.color3.toHex(), color = preset.color3.toStdColor(), style = MaterialTheme.typography.titleMedium, textAlign = TextAlign.Center, fontWeight = FontWeight.Bold)

            }
            Row(
                modifier = Modifier.padding(horizontal = 16.dp, vertical = 4.dp).fillMaxWidth(),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.SpaceAround
            ) {
                Text(text = "Smooth\n${preset.smoothSize}", textAlign = TextAlign.Center)
                Text(text = "Gain\n${preset.gain}", textAlign = TextAlign.Center)
                Text(text = "${preset.displayMode} \n${preset.animationMode}", textAlign = TextAlign.Center)
                Text(text = "FFT : ${preset.fftSize}\nSkew : ${preset.skew}", textAlign = TextAlign.Center)
                Text(text = "", textAlign = TextAlign.Center)
            }
            /*Row(
                modifier = Modifier.padding(horizontal = 16.dp, vertical = 2.dp).fillMaxWidth(),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.SpaceAround
            ) {

                Text(text = "Frequencies (${preset.frequencies.size}):", style = MaterialTheme.typography.titleMedium, textAlign = TextAlign.Center)
            }
            Row(
                modifier = Modifier.padding(horizontal = 16.dp, vertical = 8.dp).fillMaxWidth(),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.SpaceAround
            ) {

                preset.frequencies.chunked(4).forEach { chunk ->
                    Text(
                        text = chunk.joinToString("\n") { it.toString() },
                        style = MaterialTheme.typography.labelSmall,
                        textAlign = TextAlign.Center,
                        modifier = Modifier.weight(1f)
                    )
                }
            }*/
            // TitleRow(title = "Gains (${preset.frequencies.size}):")
            /*Row(
                modifier = Modifier.padding(horizontal = 16.dp, vertical = 4.dp).fillMaxWidth(),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.SpaceAround
            ) {

                preset.gains.chunked(4).forEach { chunk ->
                    Text(
                        text = chunk.joinToString("\n") { it.toString() },
                        style = MaterialTheme.typography.labelSmall,
                        textAlign = TextAlign.Center,
                        lineHeight = 1.0.em,
                        modifier = Modifier.weight(1f)
                    )
                }
            }*/

        }

    }
}

@Preview
@Composable
fun PresetCardPreview() {
    PresetCard(
        preset = Preset(index = 1u,
            name = "My awesome preset",
            smoothSize = 10u,
            gain = 1.0f,
            fps = 60u,
            color1 = Rgb888(0xFF0000u),
            color2 = Rgb888(0x00FF00u),
            color3 = Rgb888(0x0000FFu),
            fftSize = 1024u,
            frequencies = List(22) { it.toFloat() * 100f },
            gains = List(22) { 1.0f },
            skew = 0.5f,
            brightness = 1.0f,
            displayMode = DisplayMode.Spectrum,
            animationMode = AnimationMode.Full),
        onActivateClick = { /* Activate action */ },
        onDeleteClick = { /* Delete action */ },
        modifier = Modifier.padding(16.dp),
        currentPresetIndex = 1u
    )
}

@Preview
@Composable
fun PresetCardPreviewEmpty() {
    PresetCard(
        preset = null,
        onActivateClick = { /* Activate action */ },
        onDeleteClick = { /* Delete action */ },
        modifier = Modifier.padding(16.dp),
        currentPresetIndex = 1u
    )
}