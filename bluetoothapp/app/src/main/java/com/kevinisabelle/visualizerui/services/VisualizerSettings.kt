package com.kevinisabelle.visualizerui.services

import com.kevinisabelle.visualizerui.data.AnimationMode
import com.kevinisabelle.visualizerui.data.DisplayMode

data class Settings(
    var smoothSize: Int = 10,
    var gain: Float = 1.0f,
    var fps: Long = 60L,
    var color1: String = "#FF0000",
    var color2: String = "#00FF00",
    var color3: String = "#0000FF",
    var fftSize: Int = 1024,
    var frequencies: List<Float> = listOf(20f, 200f, 2000f, 20000f),
    var gains: List<Float> = listOf(1.0f, 1.0f, 1.0f, 1.0f),
    var skew: Float = 0.5f,
    var brightness: Float = 1.0f,
    var displayMode: DisplayMode = DisplayMode.Spectrum,
    var animationMode: AnimationMode = AnimationMode.Full,
    var ledsCount: Int = 60,
)