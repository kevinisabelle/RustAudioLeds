package com.kevinisabelle.visualizerui

import com.kevinisabelle.visualizerui.data.AnimationMode
import com.kevinisabelle.visualizerui.data.DisplayMode
import com.kevinisabelle.visualizerui.data.Preset
import com.kevinisabelle.visualizerui.data.Rgb888
import com.kevinisabelle.visualizerui.data.decodePreset
import com.kevinisabelle.visualizerui.data.encodePreset

class PresetTest : junit.framework.TestCase() {

    fun testEncodePresetCsv()  {
        // Create a sample Preset object
        val preset = Preset(
            index = 1u,
            name = "Test Preset",
            smoothSize = 10u,
            gain = 1.5f,
            fps = 60u,
            color1 = Rgb888(255u, 0u, 0u),  // Assuming Rgb888 constructor
            color2 = Rgb888(0u, 255u, 0u),
            color3 = Rgb888(0u, 0u, 255u),
            fftSize = 1024u,
            // 22 frequencies and gains for testing
            frequencies = listOf(100f, 200f, 300f, 400f, 500f, 600f, 700f, 800f, 900f, 1000f,
                1100f, 1200f, 1300f, 1400f, 1500f, 1600f, 1700f, 1800f, 1900f, 2000f,
                2100f, 2200f),
            // Assuming gains are also a list of floats
            gains = listOf(0.5f, 1.0f, 1.5f, 0.2f, 0.8f, 1.2f, 1.3f, 1.4f, 1.6f, 1.7f,
                1.8f, 1.9f, 2.0f, 2.1f, 2.2f, 2.3f, 2.4f, 2.5f, 2.6f, 2.7f, 2.8f, 2.9f),
            skew = 0.2f,
            brightness = 0.8f,
            displayMode = DisplayMode.entries.first(),  // Adjust based on your enum
            animationMode = AnimationMode.entries.first()
        )

        // Call the function
        val encoded = encodePreset(preset)
        val decoded = decodePreset(encoded)

        assert(decoded.name == preset.name)
        assert(decoded.smoothSize == preset.smoothSize)
        assert(decoded.gain == preset.gain)
        assert(decoded.fps == preset.fps)
        assert(decoded.color1 == preset.color1)
        assert(decoded.color2 == preset.color2)
        assert(decoded.color3 == preset.color3)
        assert(decoded.fftSize == preset.fftSize)
        assert(decoded.frequencies == preset.frequencies)
        assert(decoded.gains == preset.gains)
        assert(decoded.skew == preset.skew)
        assert(decoded.brightness == preset.brightness)
    }
}