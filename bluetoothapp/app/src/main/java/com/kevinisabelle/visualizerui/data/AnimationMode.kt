package com.kevinisabelle.visualizerui.data

enum class AnimationMode(val code: UByte) {
    Full(0u),
    FullWithMax(1u),
    Points(2u),
    FullMiddle(3u),
    FullMiddleWithMax(4u),
    PointsMiddle(5u);

    companion object {
        fun from(code: UByte) =
            entries.firstOrNull { it.code == code }
                ?: error("Unknown AnimationMode code=$code")
    }
}