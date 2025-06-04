package com.kevinisabelle.visualizerui

import android.app.Application
import com.kevinisabelle.visualizerui.data.AnimationMode
import com.kevinisabelle.visualizerui.data.DisplayMode
import com.kevinisabelle.visualizerui.services.Settings
import dagger.hilt.android.HiltAndroidApp

@HiltAndroidApp
class VisualizerApplication : Application() {

    val settings: Settings by lazy {
        Settings(
            smoothSize = 10,
            gain = 1.0f,
            fps = 60L,
            color1 = "#FF0000",
            color2 = "#00FF00",
            color3 = "#0000FF",
            fftSize = 1024,
            frequencies = listOf(20f, 200f, 2000f, 20000f),
            gains = listOf(1.0f, 1.0f, 1.0f, 1.0f),
            skew = 0.5f,
            brightness = 1.0f,
            displayMode = DisplayMode.Spectrum,
            animationMode = AnimationMode.Full
        )
    }

    override fun onCreate() {
        super.onCreate()
        // Initialize any global resources or services here
        // For example, you might want to initialize a logging library or a dependency injection framework
    }
}