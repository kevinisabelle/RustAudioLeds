package com.kevinisabelle.visualizerui.services

enum class DisplayMode {
    Spectrum,
    Oscilloscope,
    ColorGradient
}

enum class AnimationMode {
    Full,
    FullWithMax,
    Points,
    FullMiddle,
    FullMiddleWithMax,
    PointsMiddle
}

data class Settings(
    val smoothSize: Int,
    val gain: Float,
    val fps: Long,
    val color1: String,
    val color2: String,
    val color3: String,
    val fftSize: Int,
    val frequencies: List<Float>,
    val gains: List<Float>,
    val skew: Float,
    val brightness: Float,
    val displayMode: DisplayMode,
    val animationMode: AnimationMode
)