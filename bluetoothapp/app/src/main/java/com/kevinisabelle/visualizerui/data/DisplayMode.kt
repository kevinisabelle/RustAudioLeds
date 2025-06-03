package com.kevinisabelle.visualizerui.data

enum class DisplayMode(val code: UByte) {
    Spectrum(0u),
    Oscilloscope(1u),
    ColorGradient(2u);

    companion object {
        fun from(code: UByte) =
            entries.firstOrNull { it.code == code }
                ?: error("Unknown DisplayMode code=$code")
    }
}