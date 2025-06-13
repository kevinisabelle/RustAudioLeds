package com.kevinisabelle.visualizerui.data

import androidx.compose.ui.graphics.Color

/**
 * Exact 24-bit RGB triple, identical to the 3-byte payload sent over BLE.
 * Stored as three `UByte`s to avoid sign problems and make LE encoding trivial.
 */
@JvmInline
value class Rgb888(val packed: UInt) {
    fun toHex(): String {
        return String.format("#%02X%02X%02X", r.toInt(), g.toInt(), b.toInt())
    }

    fun toStdColor(): Color {
        return Color(
            red = r.toFloat() / 255f,
            green = g.toFloat() / 255f,
            blue = b.toFloat() / 255f,
            alpha = 1f
        )
    }

    val r: UByte get() = ((packed shr 16) and 0xFFu).toUByte()
    val g: UByte get() = ((packed shr  8) and 0xFFu).toUByte()
    val b: UByte get() = ( packed        and 0xFFu).toUByte()

    constructor(r: UByte, g: UByte, b: UByte) : this(
        (r.toUInt() shl 16) or (g.toUInt() shl 8) or b.toUInt()
    )

    companion object {
        fun fromStdColor(color: Color): Rgb888 {
            return Rgb888(
                r = (color.red * 255).toInt().toUByte(),
                g = (color.green * 255).toInt().toUByte(),
                b = (color.blue * 255).toInt().toUByte()
            )
        }

        fun fromHex(string: String): Rgb888 {
            if (string.length != 7 || string[0] != '#') {
                throw IllegalArgumentException("Invalid hex color format: $string")
            }
            val r = string.substring(1, 3).toUByte(16)
            val g = string.substring(3, 5).toUByte(16)
            val b = string.substring(5, 7).toUByte(16)
            return Rgb888(r, g, b)
        }
    }
}