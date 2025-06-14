﻿package com.kevinisabelle.visualizerui.ui.components

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.DisabledByDefault
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.icons.filled.Save
import androidx.compose.material3.Button
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue //
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.rotate
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.lifecycle.viewmodel.compose.viewModel
import com.kevinisabelle.visualizerui.data.AnimationMode
import com.kevinisabelle.visualizerui.data.DisplayMode
import com.kevinisabelle.visualizerui.data.Preset
import com.kevinisabelle.visualizerui.data.Rgb888
import com.kevinisabelle.visualizerui.services.Settings

@Composable
fun DeviceSettings(
    settings: Settings,
    presets: List<Preset>,
    onSetColor1: (Rgb888) -> Unit,
    onSetColor2: (Rgb888) -> Unit,
    onSetColor3: (Rgb888) -> Unit,
    onSetFps: (Long) -> Unit = { }, // Default empty function for fps
    onSetBrightness: (Float) -> Unit = { }, // Default empty function for brightness
    onSetDisplayMode: (DisplayMode) -> Unit = { }, // Default empty function for display mode
    onSetAnimationMode: (AnimationMode) -> Unit = { }, // Default empty function for animation mode
    onSetFftSize: (Int) -> Unit = { }, // Default empty function for FFT size
    onSetGain: (Float) -> Unit,
    onSetFrequencyGain: (Int, Float) -> Unit, // Default empty function for frequency gain
    onSetSmoothSize: (Int) -> Unit,
    onSetSkew: (Float) -> Unit = { }, // Default empty function for skew
    onRefreshClick: () -> Unit = { }, // Default empty function for refresh action
    onSaveClick: (String) -> Unit = { }, // Default empty function for save action,
    onNewPresetClick: () -> Unit = { } // Default empty function for new preset action
) {
    var showSaveConfirmationDialog by remember { mutableStateOf(false) }
    val scrollState = rememberScrollState()
    val currentPreset = presets.find { it.index == settings.currentPresetIndex.toUByte() }
    Column(
        modifier = Modifier.padding(horizontal = 20.dp).verticalScroll(scrollState),
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
                    contentDescription = "Reload settings"
                )
            }
            Text(
                text = "Settings",
                style = MaterialTheme.typography.titleLarge,
                textAlign = TextAlign.Center,
                modifier = Modifier.weight(1f)
            )
            IconButton(
                onClick = { showSaveConfirmationDialog = true },
                modifier = Modifier.padding(start = 8.dp)
            ) {
                Icon(
                    imageVector = Icons.Default.Save,
                    contentDescription = "Save presets settings"
                )
            }

            if (showSaveConfirmationDialog) {
                NameInputDialog(
                    initialName = currentPreset?.name ?: "",
                    onDismiss = { showSaveConfirmationDialog = false },
                    onConfirm = { name ->
                        showSaveConfirmationDialog = false
                        onSaveClick(name)
                    }
                )
            }
        }
        Row(verticalAlignment = Alignment.CenterVertically)
        {
            if (currentPreset != null)
            {
                Text(
                    text = "Current Preset: ${currentPreset.name} (${settings.currentPresetIndex})",
                    style = MaterialTheme.typography.titleSmall,
                    textAlign = TextAlign.Center,
                    modifier = Modifier.weight(1f)
                )

                IconButton(
                    onClick = { onNewPresetClick() },
                    modifier = Modifier.rotate(180f)
                ) {
                    Icon(
                        imageVector = Icons.Default.DisabledByDefault,
                        contentDescription = "New preset"
                    )
                }
            } else {
                Text(
                    text = "No preset selected",
                    style = MaterialTheme.typography.titleSmall,
                    textAlign = TextAlign.Center,
                    modifier = Modifier.weight(1f)
                )
            }
        }
        Row(
            verticalAlignment = Alignment.CenterVertically,
            modifier = Modifier.height(300.dp))
        {
            Column(
                horizontalAlignment = Alignment.CenterHorizontally,
                modifier = Modifier.weight(0.25f).padding(end = 8.dp)
            )
            {
                Text(
                    text = "Color 1",
                    style = MaterialTheme.typography.titleMedium,
                    textAlign = TextAlign.Center
                )

                ColorPicker(
                    color = settings.color1,
                    modifier = Modifier.width(100.dp).height(300.dp).padding(top = 8.dp),
                    onColorSelected = { newColor -> onSetColor1(newColor) }
                )
            }
            Column(
                horizontalAlignment = androidx.compose.ui.Alignment.CenterHorizontally,
                modifier = Modifier.weight(0.25f).padding(horizontal = 8.dp)
            )
            {
                Text(
                    text = "Color 2",
                    style = MaterialTheme.typography.titleMedium,
                    textAlign = TextAlign.Center
                )

                ColorPicker(
                    color = settings.color2,
                    modifier = Modifier.width(100.dp).height(300.dp).padding(top = 8.dp),
                    onColorSelected = { newColor -> onSetColor2(newColor) }
                )
            }
            Column(
                horizontalAlignment = androidx.compose.ui.Alignment.CenterHorizontally,
                modifier = Modifier.weight(0.25f).padding(start = 8.dp)
            )
            {
                Text(
                    text = "Color 3",
                    style = MaterialTheme.typography.titleMedium,
                    textAlign = TextAlign.Center
                )

                ColorPicker(
                    color = settings.color3,
                    modifier = Modifier.width(100.dp).height(300.dp).padding(top = 8.dp),
                    onColorSelected = { newColor -> onSetColor3(newColor) }
                )
            }
        }
        TitleRow(title = "Brightness ${settings.brightness.toString().take(4)}")
        Row(verticalAlignment = Alignment.CenterVertically) {
            Slider(
                value = settings.brightness,
                onValueChange = { newValue ->
                    onSetBrightness(newValue)
                },
                valueRange = 0f..1f,
                modifier = Modifier.weight(1f)
            )
        }
        TitleRow(title="Mode")
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(10.dp, Alignment.CenterHorizontally)
        )
        {
            Button(
                onClick = {
                    onSetDisplayMode(DisplayMode.Spectrum)
                },
                enabled = settings.displayMode != DisplayMode.Spectrum,
                modifier = Modifier.weight(0.33f)
            ) {
                Text(text = "Spectrum")
            }
            /*Button(
                onClick = {
                    onSetDisplayMode(DisplayMode.Oscilloscope)
                },
                enabled = settings.displayMode != DisplayMode.Oscilloscope,
                modifier = Modifier.weight(0.33f)
            ) {
                Text(text = "Wave")
            }*/
            Button(
                onClick = {
                    onSetDisplayMode(DisplayMode.ColorGradient)
                },
                enabled = settings.displayMode != DisplayMode.ColorGradient,
                modifier = Modifier.weight(0.33f)
            ) {
                Text(text = "Color")
            }

        }
        TitleRow(title="Animation")
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(10.dp, Alignment.CenterHorizontally)
        )
        {
            Button(
                onClick = {
                    onSetAnimationMode(AnimationMode.Full)
                },
                enabled = settings.animationMode != AnimationMode.Full,
                modifier = Modifier.weight(0.5f)
            ) {
                Text(text = "Full")
            }
            Button(
                onClick = {
                    onSetAnimationMode(AnimationMode.FullWithMax)
                },
                enabled = settings.animationMode != AnimationMode.FullWithMax,
                modifier = Modifier.weight(0.5f)
            ) {
                Text(text = "Full w/ Max")
            }
        }
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(10.dp, Alignment.CenterHorizontally)
        )
        {
            Button(
                onClick = {
                    onSetAnimationMode(AnimationMode.FullMiddle)
                },
                enabled = settings.animationMode != AnimationMode.FullMiddle,
                modifier = Modifier.weight(0.5f)
            ) {
                Text(text = "Full Middle")
            }
            Button(
                onClick = {
                    onSetAnimationMode(AnimationMode.FullMiddleWithMax)
                },
                enabled = settings.animationMode != AnimationMode.FullMiddleWithMax,
                modifier = Modifier.weight(0.5f)
            ) {
                Text(text = "Full Middle w/ Max")
            }
        }
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(10.dp, Alignment.CenterHorizontally)
        )
        {
            /*Button(
                onClick = {
                    onSetAnimationMode(AnimationMode.PointsMiddle)
                },
                enabled = settings.animationMode != AnimationMode.PointsMiddle,
                modifier = Modifier.weight(0.5f)
            ) {
                Text(text = "Points Middle")
            }*/
            Button(
                onClick = {
                    onSetAnimationMode(AnimationMode.Points)
                },
                enabled = settings.animationMode != AnimationMode.Points,
                modifier = Modifier.weight(0.5f)
            ) {
                Text(text = "Points")
            }
        }
        TitleRow(title = "Gain ${settings.gain.toString().take(4)}")
        Row(verticalAlignment = Alignment.CenterVertically) {
            Slider(
                value = settings.gain,
                onValueChange = { newValue ->
                    onSetGain(newValue)
                },
                valueRange = 0.5f..50f,
                modifier = Modifier.weight(1f)
            )
        }
        TitleRow(title = "Smooth Size ${settings.smoothSize}")
        Row(verticalAlignment = Alignment.CenterVertically) {
            Slider(
                value = settings.smoothSize.toFloat(),
                onValueChange = { newValue ->
                    onSetSmoothSize(newValue.toInt())
                },
                valueRange = 1f..50f,
                modifier = Modifier.weight(1f)
            )
        }
        TitleRow(title = "Skew ${settings.skew.toString().take(4)}")
        Row(verticalAlignment = Alignment.CenterVertically) {
            Slider(
                value = settings.skew.toFloat(),
                onValueChange = { newValue ->
                    onSetSkew(newValue)
                },
                valueRange = 0.1f..1.5f,
                modifier = Modifier.weight(1f)
            )
        }
        TitleRow(title = "FPS ${settings.fps}")
        Row(verticalAlignment = Alignment.CenterVertically) {
            Slider(
                value = settings.fps.toFloat(),
                onValueChange = { newValue ->
                    onSetFps(newValue.toLong())
                },
                valueRange = 10f..60f,
                steps = 49,
                modifier = Modifier.weight(1f)
            )
        }
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.Center
        )
        {
            Text(
                text = "FFT",
                style = MaterialTheme.typography.titleMedium,
                textAlign = TextAlign.Center,
                modifier = Modifier.weight(1f))
        }
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(10.dp, Alignment.CenterHorizontally)
        ) {
            Button(
                onClick = {
                    onSetFftSize(1024)
                },
                enabled = settings.fftSize.toUInt() != 1024u, // Disable if already set to 256
                modifier = Modifier.weight(0.2f)
            ) {
                Text(text = "1024") // Example button text, replace with actual logic
            }
            Button(
                onClick = {
                    onSetFftSize(2048)
                },
                enabled = settings.fftSize.toUInt() != 2048u, // Disable if already set to 256
                modifier = Modifier.weight(0.2f)
            ) {
                Text(text = "2048") // Example button text, replace with actual logic
            }
            Button(
                onClick = {
                    onSetFftSize(4096)
                },
                enabled = settings.fftSize.toUInt() != 4096u, // Disable if already set to 4096
                modifier = Modifier.weight(0.2f)
            ) {
                Text(text = "4096") // Example button text, replace with actual logic
            }
            Button(
                onClick = {
                    onSetFftSize(8192)
                },
                enabled = settings.fftSize.toUInt() != 8192u, // Disable if already set to 8192
                modifier = Modifier.weight(0.2f)
            ) {
                Text(text = "8192") // Example button text, replace with actual logic
            }

        }
        Row(
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.Center
        )
        {
            Text(
                text = "Gains Per Frequency",
                style = MaterialTheme.typography.titleMedium,
                textAlign = TextAlign.Center,
                modifier = Modifier.weight(1f))
        }
        for (y in 0 until 12) {


            Row(
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.SpaceAround,
                modifier = Modifier.height(70.dp)
            ) {
                for (i in (y*2) until (settings.frequencies.size.coerceAtMost((y+1)*2))) {
                    Column(
                        horizontalAlignment =Alignment.CenterHorizontally,
                        modifier = Modifier.weight(1.0f / settings.frequencies.size.toFloat())
                            .height(120.dp).padding(horizontal = 3.dp)
                    ) {
                        Text(
                            text = "${settings.frequencies[i].toInt()} Hz",
                            style = MaterialTheme.typography.labelSmall
                        )

                        Slider(
                            value = settings.gains[i],
                            onValueChange = { newValue ->
                                onSetFrequencyGain(i, newValue)
                            },
                            valueRange = 0.5f..4.0f,
                            modifier = Modifier.height(20.dp).padding(top = 6.dp),
                        )
                        Text(
                            text = settings.gains[i].toString().take(4),
                            style = MaterialTheme.typography.labelSmall,
                            lineHeight = 10.sp,
                            modifier = Modifier.padding(top = 8.dp) // Padding for gain label
                        )
                    }
                }
            }
        }
    }
}

@Preview
@Composable
fun DeviceSettingsPreview() {

    var currentSettings by remember {
        mutableStateOf(
            Settings( // Initial settings
                smoothSize = 10,
                gain = 1.0f,
                fps = 60L,
                color1 = Rgb888.fromStdColor(Color.Blue),
                color2 = Rgb888.fromStdColor(Color.Blue),
                color3 = Rgb888.fromStdColor(Color.Blue),
                fftSize = 4096u,
                frequencies = listOf(41.0f, 55.0f, 65.0f, 82.0f, 110.0f, 146.0f, 220.0f, 261.0f, 329.0f, 392.0f,
                    440.0f, 523.0f, 880.0f, 987.0f, 2000.0f, 3000.0f, 4000.0f, 5000.0f, 6000.0f, 7500.0f,
                    9000.0f, 13000.0f),
                gains = listOf(1.3f, 1.2f, 1.1f, 1.0f, 1.0f, 1.0f, 1.0f, 0.85f, 0.75f, 0.75f,
                    0.75f, 0.75f, 0.75f, 0.75f, 1.0f, 1.0f, 1.0f, 1.0f, 1.2f, 3.0f,
                    4.0f, 4.0f),
                skew = 0.5f,
                brightness = 1.0f,
                displayMode = DisplayMode.Spectrum,
                animationMode = AnimationMode.Full
            )
        )
    }

    DeviceSettings(
        settings = currentSettings,
        onSetGain = { newGain ->
            currentSettings = currentSettings.copy(gain = newGain.toFloat())
        },
        onSetFrequencyGain = { index, newGain ->
            val updatedGains = currentSettings.gains.toMutableList()
            updatedGains[index] = newGain
            currentSettings = currentSettings.copy(gains = updatedGains)
        },
        onSetFftSize = { newFftSize ->
            currentSettings = currentSettings.copy(fftSize = newFftSize.toUShort())
        },
        onSetSmoothSize = { newSmoothSize ->
            currentSettings = currentSettings.copy(smoothSize = newSmoothSize)
        },
        onSetSkew = { newSkew ->
            currentSettings = currentSettings.copy(skew = newSkew)
        },
        onSetFps = { newFps ->
            currentSettings = currentSettings.copy(fps = newFps)
        },
        onSetBrightness = { newBrightness ->
            currentSettings = currentSettings.copy(brightness = newBrightness)
        },
        onSetDisplayMode = { newDisplayMode ->
            currentSettings = currentSettings.copy(displayMode = newDisplayMode)
        },
        onSetAnimationMode = { newAnimationMode ->
            currentSettings = currentSettings.copy(animationMode = newAnimationMode)
        },
        onSetColor1 = { newColor ->
            currentSettings = currentSettings.copy(color1 = newColor)
        },
        onSetColor2 = { newColor ->
            currentSettings = currentSettings.copy(color2 = newColor)
        },
        onSetColor3 = { newColor ->
            currentSettings = currentSettings.copy(color3 = newColor)
        },
        onRefreshClick = {
            // Handle refresh action
            println("Refresh clicked")
        },
        presets = listOf(
            Preset(
                index = 0u,
                name = "Default Preset",
                smoothSize = 10u,
                gain = 1.0f,
                fps = 60u,
                color1 = Rgb888.fromStdColor(Color.Blue),
                color2 = Rgb888.fromStdColor(Color.Green),
                color3 = Rgb888.fromStdColor(Color.Red),
                fftSize = 4096u,
                frequencies = listOf(20f, 200f, 2000f, 20000f),
                gains = List(22) { 1.0f },
                skew = 0.5f,
                brightness = 1.0f,
                displayMode = DisplayMode.Spectrum,
                animationMode = AnimationMode.Full
            )
        ),
        onNewPresetClick = {
            currentSettings.currentPresetIndex = 255

        }
    )
}