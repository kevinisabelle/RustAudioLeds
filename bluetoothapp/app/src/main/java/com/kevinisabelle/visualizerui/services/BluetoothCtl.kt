package com.kevinisabelle.visualizerui.services


class BluetoothCtl {

    // We need a function to get the VisualizerSettings
    fun getGain(): Float {
        // This function should return the gain value from the VisualizerSettings
        // For now, we will return a default value
        return 1.0f
    }

    fun getFps(): Long {
        // This function should return the fps value from the VisualizerSettings
        // For now, we will return a default value
        return 60L
    }

    fun setGain(value: Float) {
        // This function should set the gain value in the VisualizerSettings
        // For now, we will just print the value
        println("Setting gain to $value")
    }

    fun setFps(value: Long) {
        // This function should set the fps value in the VisualizerSettings
        // For now, we will just print the value
        println("Setting fps to $value")
    }

}