package com.kevinisabelle.visualizerui.services

import com.kevinisabelle.visualizerui.data.AnimationMode
import com.kevinisabelle.visualizerui.data.DisplayMode

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