package com.kevinisabelle.visualizerui.ui.components

import androidx.compose.foundation.layout.Column
import androidx.compose.runtime.Composable
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.width
import androidx.compose.material3.Button
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.runtime.getValue // Import getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue // Import setValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.rotate
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import com.kevinisabelle.visualizerui.data.AnimationMode
import com.kevinisabelle.visualizerui.data.DisplayMode
import com.kevinisabelle.visualizerui.services.Settings
// kotlin.text.toFloat is not needed here as newGain is already Int and toFloat() is a standard library function

@Composable
fun DeviceSettings(
    settings: Settings,
    onSetGain: (Int) -> Unit,
    onSetFrequencyGain: (Int, Float) -> Unit, // Default empty function for frequency gain
    onSetSmoothSize: (Int) -> Unit,
    onSetSkew: (Float) -> Unit = { }, // Default empty function for skew
) {
    Column(
        modifier = Modifier.padding(4.dp),
        verticalArrangement = androidx.compose.foundation.layout.Arrangement.spacedBy(8.dp)
    ) {
        Row(
            verticalAlignment = androidx.compose.ui.Alignment.CenterVertically
        )
        {
            Text(
                text = "Mode"
            , modifier = Modifier.weight(0.1f))
            Button(
                onClick = {
                    // Handle display mode change
                    // This is a placeholder; actual implementation may vary
                    println("Display Mode button clicked")
                },
                enabled = settings.displayMode != DisplayMode.Spectrum, // Disable if already set to Spectrum
                modifier = Modifier.weight(0.3f)
            ) {
                Text(text = "Spectrum") // Example button text, replace with actual logic
            }
            Button(
                onClick = {
                    // Handle display mode change
                    // This is a placeholder; actual implementation may vary
                    println("Display Mode button clicked")
                },
                enabled = settings.displayMode != DisplayMode.Oscilloscope, // Disable if already set to Waveform
                modifier = Modifier.weight(0.3f)
            ) {
                Text(text = "Wave") // Example button text, replace with actual logic
            }
            Button(
                onClick = {
                    // Handle display mode change
                    // This is a placeholder; actual implementation may vary
                    println("Display Mode button clicked")
                },
                enabled = settings.displayMode != DisplayMode.ColorGradient, // Disable if already set to Waveform
                modifier = Modifier.weight(0.3f)
            ) {
                Text(text = "Color") // Example button text, replace with actual logic
            }

        }
        Row(verticalAlignment = androidx.compose.ui.Alignment.CenterVertically) {
            Text(
                text = "Gain ${settings.gain.toInt()}",
                modifier = Modifier.weight(0.25f)
            )
            Slider(
                value = settings.gain,
                onValueChange = { newValue ->
                    onSetGain(newValue.toInt())
                },
                valueRange = 1f..100f,
                steps = 99,
                modifier = Modifier.weight(0.75f)
            )
        }
        Row(verticalAlignment = androidx.compose.ui.Alignment.CenterVertically) {
            Text(
                text = "Smooth Size ${settings.smoothSize}",
                modifier = Modifier.weight(0.25f)
            )
            Slider(
                value = settings.smoothSize.toFloat(),
                onValueChange = { newValue ->
                    onSetSmoothSize(newValue.toInt())
                },
                valueRange = 1f..100f,
                steps = 99,
                modifier = Modifier.weight(0.75f)
            )
        }
        Row(verticalAlignment = androidx.compose.ui.Alignment.CenterVertically) {
            Text(
                text = "Skew ${settings.skew.toString().take(4)}",
                modifier = Modifier.weight(0.25f)
            )
            Slider(
                value = settings.skew.toFloat(),
                onValueChange = { newValue ->
                    onSetSkew(newValue)
                },
                valueRange = 0.1f..5.0f,
                modifier = Modifier.weight(0.75f)
            )
        }
        Row(verticalAlignment = androidx.compose.ui.Alignment.CenterVertically) {
            Text(
                text = "FFT",
                modifier = Modifier.weight(0.1f)
            )
            Button(
                onClick = {
                    // Handle FFT size change
                    // This is a placeholder; actual implementation may vary
                    println("FFT Size button clicked")
                },
                enabled = settings.fftSize != 1024, // Disable if already set to 256
                modifier = Modifier.weight(0.2f)
            ) {
                Text(text = "1024") // Example button text, replace with actual logic
            }
            Button(
                onClick = {
                    // Handle FFT size change
                    // This is a placeholder; actual implementation may vary
                    println("FFT Size button clicked")
                },
                enabled = settings.fftSize != 2048, // Disable if already set to 256
                modifier = Modifier.weight(0.2f)
            ) {
                Text(text = "2048") // Example button text, replace with actual logic
            }
            Button(
                onClick = {
                    // Handle FFT size change
                    // This is a placeholder; actual implementation may vary
                    println("FFT Size button clicked")
                },
                enabled = settings.fftSize != 4096, // Disable if already set to 4096
                modifier = Modifier.weight(0.2f)
            ) {
                Text(text = "4096") // Example button text, replace with actual logic
            }
            Button(
                onClick = {
                    // Handle FFT size change
                    // This is a placeholder; actual implementation may vary
                    println("FFT Size button clicked")
                },
                enabled = settings.fftSize != 8192, // Disable if already set to 8192
                modifier = Modifier.weight(0.2f)
            ) {
                Text(text = "8192") // Example button text, replace with actual logic
            }

        }
        Row()
        {
            Text(
                text = "Gains Per Frequency",
                style = androidx.compose.material3.MaterialTheme.typography.titleMedium,
                modifier = Modifier.weight(1f)
            )
        }
        Row(
            verticalAlignment = androidx.compose.ui.Alignment.CenterVertically,
            modifier = Modifier.height(80.dp)) {
            for (i in 0 until settings.frequencies.size/2) {
                Column(
                    horizontalAlignment = androidx.compose.ui.Alignment.CenterHorizontally,
                    modifier = Modifier.weight(1.0f / settings.frequencies.size.toFloat()).height(120.dp)
                ) {
                    Text(
                        text = "${settings.frequencies[i].toInt()}",
                        // Caption
                        style = androidx.compose.material3.MaterialTheme.typography.labelSmall,
                        // Line height
                        lineHeight = 10.sp,
                    )

                    Slider(
                        value = settings.gains[i],
                        onValueChange = { newValue ->
                            // Handle gain change for each frequency
                            // This is a placeholder; actual implementation may vary
                            println("Gain for frequency ${settings.frequencies[i]} Hz set to $newValue")
                            onSetFrequencyGain(i, newValue)
                        },
                        modifier = Modifier.padding(top = 16.dp).height(2.dp).width(200.dp).rotate(-90f),
                        valueRange = 0.5f..5.0f,
                    )
                    Text(
                        text = "${settings.gains[i].toString().take(4)}",
                        // Caption
                        style = androidx.compose.material3.MaterialTheme.typography.labelSmall,
                        // Line height

                        lineHeight = 10.sp,
                        modifier = Modifier.padding(top = 18.dp) // Padding for gain label
                    )
                }
            }
        }

        Row(
            verticalAlignment = androidx.compose.ui.Alignment.CenterVertically,
            modifier = Modifier.height(120.dp)) {
            for (i in settings.frequencies.size/2 until settings.frequencies.size) {
                Column(
                    horizontalAlignment = androidx.compose.ui.Alignment.CenterHorizontally,
                    modifier = Modifier.weight(1.0f / settings.frequencies.size.toFloat()).height(120.dp)
                ) {
                    Text(
                        text = "${settings.frequencies[i].toInt()}",
                        // Caption
                        style = androidx.compose.material3.MaterialTheme.typography.labelSmall,
                        // Line height
                        lineHeight = 10.sp,
                    )

                    Slider(
                        value = settings.gains[i],
                        onValueChange = { newValue ->
                            // Handle gain change for each frequency
                            // This is a placeholder; actual implementation may vary
                            println("Gain for frequency ${settings.frequencies[i]} Hz set to $newValue")
                            onSetFrequencyGain(i, newValue)
                        },
                        modifier = Modifier.padding(top = 16.dp).height(2.dp).width(200.dp).rotate(-90f),
                        valueRange = 0.5f..5.0f,
                    )
                    Text(
                        text = "${settings.gains[i].toString().take(4)}",
                        // Caption
                        style = androidx.compose.material3.MaterialTheme.typography.labelSmall,
                        // Line height

                        lineHeight = 10.sp,
                        modifier = Modifier.padding(top = 16.dp) // Padding for gain label
                    )
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
                color1 = "#FF0000",
                color2 = "#00FF00",
                color3 = "#0000FF",
                fftSize = 1024,
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
            // Handle gain change
            // Ensure Settings is a data class for .copy() to work
            currentSettings = currentSettings.copy(gain = newGain.toFloat())
            println("Gain set to: $newGain")
        },
        onSetFrequencyGain = { index, newGain ->
            // Handle frequency gain change
            // Ensure Settings is a data class for .copy() to work
            val updatedGains = currentSettings.gains.toMutableList()
            updatedGains[index] = newGain
            currentSettings = currentSettings.copy(gains = updatedGains)
            println("Frequency gain for index $index set to: $newGain")
        },
        onSetSmoothSize = { newSmoothSize ->
            // Handle smooth size change
            // Ensure Settings is a data class for .copy() to work
            currentSettings = currentSettings.copy(smoothSize = newSmoothSize)
            println("Smooth Size set to: $newSmoothSize")
        },
        onSetSkew = { newSkew ->
            // Handle skew change
            // Ensure Settings is a data class for .copy() to work
            currentSettings = currentSettings.copy(skew = newSkew)
            println("Skew set to: $newSkew")
        }
    )
}