package com.kevinisabelle.visualizerui.data

/**
 * Exact 24-bit RGB triple, identical to the 3-byte payload sent over BLE.
 * Stored as three `UByte`s to avoid sign problems and make LE encoding trivial.
 */
@JvmInline
value class Rgb888(val packed: UInt) {
    fun toHex(): String {
        return String.format("#%02X%02X%02X", r.toInt(), g.toInt(), b.toInt())
    }

    val r: UByte get() = ((packed shr 16) and 0xFFu).toUByte()
    val g: UByte get() = ((packed shr  8) and 0xFFu).toUByte()
    val b: UByte get() = ( packed        and 0xFFu).toUByte()

    constructor(r: UByte, g: UByte, b: UByte) : this(
        (r.toUInt() shl 16) or (g.toUInt() shl 8) or b.toUInt()
    )
}