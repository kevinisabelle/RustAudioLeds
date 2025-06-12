package com.kevinisabelle.visualizerui.services

import androidx.compose.ui.graphics.Color
import com.kevinisabelle.visualizerui.data.AnimationMode
import com.kevinisabelle.visualizerui.data.DisplayMode
import com.kevinisabelle.visualizerui.data.Rgb888

data class Settings(
    var smoothSize: Int = 10,
    var gain: Float = 1.0f,
    var fps: Long = 60L,
    var color1: Rgb888 = Rgb888.fromStdColor(Color.Blue),
    var color2: Rgb888 = Rgb888.fromStdColor(Color.Green),
    var color3: Rgb888 = Rgb888.fromStdColor(Color.Red),
    var fftSize: UShort = 1024u,
    var frequencies: List<Float> = listOf(20f, 200f, 2000f, 20000f),
    var gains: List<Float> = listOf(1.0f, 1.0f, 1.0f, 1.0f),
    var skew: Float = 0.5f,
    var brightness: Float = 1.0f,
    var displayMode: DisplayMode = DisplayMode.Spectrum,
    var animationMode: AnimationMode = AnimationMode.Full,
    var ledsCount: Int = 60,
    var currentPresetIndex: Int = 0,
)