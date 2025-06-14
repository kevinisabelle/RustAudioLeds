package com.kevinisabelle.visualizerui

import android.app.Application
import androidx.compose.ui.graphics.Color
import com.kevinisabelle.visualizerui.data.AnimationMode
import com.kevinisabelle.visualizerui.data.DisplayMode
import com.kevinisabelle.visualizerui.data.Rgb888
import com.kevinisabelle.visualizerui.services.Settings
import dagger.hilt.android.HiltAndroidApp

@HiltAndroidApp
class VisualizerApplication : Application() {

    override fun onCreate() {
        super.onCreate()
        // Initialize any global resources or services here
        // For example, you might want to initialize a logging library or a dependency injection framework
    }
}