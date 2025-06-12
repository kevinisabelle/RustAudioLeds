package com.kevinisabelle.visualizerui.data

import com.kevinisabelle.visualizerui.ble.toByte
import com.kevinisabelle.visualizerui.ble.toBytes
import com.kevinisabelle.visualizerui.ble.toLeBytes
import com.kevinisabelle.visualizerui.ble.toUShort

data class Preset(
    val index: UByte,
    val name: String,
    val smoothSize: UShort,
    val gain: Float,
    val fps: UShort,
    val color1: Rgb888,
    val color2: Rgb888,
    val color3: Rgb888,
    val fftSize: UShort,
    val frequencies: List<Float>,
    val gains: List<Float>,
    val skew: Float,
    val brightness: Float,
    val displayMode: DisplayMode,
    val animationMode: AnimationMode
) {
    companion object {
        const val NAME_MAX_LENGTH = 16
    }
}

private fun ByteArray.toUInt(): UInt {
    require(size <= 4) { "ByteArray too large to convert to UInt" }
    var result = 0U
    for (i in indices) {
        result = (result shl 8) or this[i].toUByte().toUInt()
    }
    return result
}

private fun ByteArray.toFloat(): Float {
    require(size == 4) { "ByteArray must be exactly 4 bytes to convert to Float" }
    return java.nio.ByteBuffer.wrap(this).float
}

fun decodePreset(data: ByteArray): Preset {
    require(data.size == 24 + 3 * 3 + 2 * 22 + 2 + 4 + 4 + 1 + 1) { "Invalid postcard preset data size" }

    val index = data[0].toUByte()
    val name = data.copyOfRange(1, 17).decodeToString().trimEnd('\u0000')
    require(name.length <= Preset.NAME_MAX_LENGTH) { "Preset name exceeds maximum length" }

    val smoothSize = data.sliceArray(17..18).toUShort()
    val gain = data.sliceArray(19..22).toFloat()
    val fps = data.sliceArray(23..24).toUShort()

    val color1 = Rgb888(data.sliceArray(25..27).toUInt())
    val color2 = Rgb888(data.sliceArray(28..30).toUInt())
    val color3 = Rgb888(data.sliceArray(31..33).toUInt())

    val fftSize = data.sliceArray(34..35).toUShort()

    val frequencies = List(22) { i ->
        data.sliceArray(36 + i * 4 until 40 + i * 4).toFloat()
    }

    val gains = List(22) { i ->
        data.sliceArray(100 + i * 4 until 104 + i * 4).toFloat()
    }

    val skew = data.sliceArray(164..167).toFloat()
    val brightness = data.sliceArray(168..171).toFloat()

    val displayMode = DisplayMode.from(data[172].toUByte())
    val animationMode = AnimationMode.from(data[173].toUByte())

    return Preset(
        index,
        name,
        smoothSize,
        gain,
        fps,
        color1,
        color2,
        color3,
        fftSize,
        frequencies,
        gains,
        skew,
        brightness,
        displayMode,
        animationMode
    )
}

fun encodePreset(preset: Preset): ByteArray {
    val nameBytes = preset.name.encodeToByteArray().copyOf(Preset.NAME_MAX_LENGTH)
    val data = ByteArray(24 + 3 * 3 + 2 * 22 + 2 + 4 + 4 + 1 + 1)

    data[0] = preset.index.toByte()
    System.arraycopy(nameBytes, 0, data, 1, nameBytes.size)

    data[17] = preset.smoothSize.toUShort().toByte()
    data[18] = (preset.smoothSize.toUInt() shr 8).toByte()

    System.arraycopy(preset.gain.toLeBytes(), 0, data, 19, 4)
    System.arraycopy(preset.fps.toUShort().toLeBytes(), 0, data, 23, 2)

    System.arraycopy(preset.color1.toBytes(), 0, data, 25, 3)
    System.arraycopy(preset.color2.toBytes(), 0, data, 28, 3)
    System.arraycopy(preset.color3.toBytes(), 0, data, 31, 3)

    System.arraycopy(preset.fftSize.toUShort().toLeBytes(), 0, data, 34, 2)

    for (i in preset.frequencies.indices) {
        System.arraycopy(preset.frequencies[i].toLeBytes(), 0, data, 36 + i * 4, 4)
    }

    for (i in preset.gains.indices) {
        System.arraycopy(preset.gains[i].toLeBytes(), 0, data, 100 + i * 4, 4)
    }

    System.arraycopy(preset.skew.toLeBytes(), 0, data, 164, 4)
    System.arraycopy(preset.brightness.toLeBytes(), 0, data, 168, 4)

    data[172] = preset.displayMode.toByte()
    data[173] = preset.animationMode.toByte()

    return data
}