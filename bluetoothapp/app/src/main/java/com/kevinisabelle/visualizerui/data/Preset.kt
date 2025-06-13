package com.kevinisabelle.visualizerui.data

import com.kevinisabelle.visualizerui.ble.toByte
import com.kevinisabelle.visualizerui.ble.toBytes
import com.kevinisabelle.visualizerui.ble.toFloatLe
import com.kevinisabelle.visualizerui.ble.toLeBytes
// import com.kevinisabelle.visualizerui.ble.toUShort
import com.kevinisabelle.visualizerui.services.Settings
import java.nio.ByteBuffer
import java.nio.ByteOrder

const val PRESET_PAYLOAD_SIZE =
        1 /*index [0]*/ +
        16 /*name [1]*/ +
        2 /*smoothSize [17]*/ +
        4 /*gain [19]*/ +
        2 /*fps [23]*/ +
        3 /*color1 [25]*/ +
        3 /*color2 [28]*/ +
        3 /*color3 [31]*/ +
        2 /*fftSize [34]*/ +
        22 * 4 /*frequencies [36]*/ +
        22 * 4 /*gains [100]*/ +
        4 /*skew [164]*/ +
        4 /*brightness [168]*/ +
        1 /*displayMode [172]*/ +
        1 /*animationMode [173]*/

data class Preset(
    var index: UByte,
    var name: String, // Must be <= Preset.NAME_MAX_LENGTH
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
        fun fromSettings(settings: Settings?) : Preset {

            if (settings == null) {
                throw IllegalArgumentException("Settings cannot be null")
            }

            return Preset(
                index = settings.currentPresetIndex.toUByte(),
                name = "Preset ${settings.currentPresetIndex}",
                smoothSize = settings.smoothSize.toUShort(),
                gain = settings.gain,
                fps = settings.fps.toUShort(),
                color1 = settings.color1,
                color2 = settings.color2,
                color3 = settings.color3,
                fftSize = settings.fftSize,
                frequencies = settings.frequencies,
                gains = settings.gains,
                skew = settings.skew,
                brightness = settings.brightness,
                displayMode = settings.displayMode,
                animationMode = settings.animationMode
            )

        }

        const val NAME_MAX_LENGTH = 16
    }
}

data class PresetEntry(
    val index: UByte,
    val name: String
) {

    companion object {
        const val NAME_MAX_LENGTH = 16

        fun fromByteArray(data: ByteArray): PresetEntry {
            require(data.size == 17) { "Invalid preset entry data size" }
            val index = data[0].toUByte()
            val name = data.copyOfRange(1, 17).decodeToString().trimEnd('\u0000')
            require(name.length <= NAME_MAX_LENGTH) { "Preset entry name exceeds maximum length" }
            return PresetEntry(index, name)
        }
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

private fun ByteArray.toUShort(): UShort {
    require(size == 2) { "ByteArray must be exactly 2 bytes to convert to UShort" }
    return ByteBuffer.wrap(this).short.toUShort()
}

private fun ByteArray.toFloat(): Float {
    require(size == 4) { "ByteArray must be exactly 4 bytes to convert to Float" }
    return ByteBuffer.wrap(this).float.toFloat()
}

fun decodePreset(data: ByteArray): Preset {
    // require(data.size == PRESET_PAYLOAD_SIZE) { "Invalid postcard preset data size" }

    val index = data[0].toUByte()
    val name = data.copyOfRange(1, 17).decodeToString().trimEnd('\u0000')
    require(name.length <= Preset.NAME_MAX_LENGTH) { "Preset name exceeds maximum length" }

    val smoothSize = data.sliceArray(17..18).toUShort()
    val gain = data.sliceArray(19..22).toFloat()
    val fps = data.sliceArray(23..24).toUShort()

    val color1 = Rgb888(data.sliceArray(25..27).toUInt())
    val color2 = Rgb888(data.sliceArray(28..30).toUInt())
    val color3 = Rgb888(data.sliceArray(31..33).toUInt())

    // The fftSize is stored as a 2-byte unsigned short (little-endian).
    val fftSize = data.sliceArray(34..35).toUShort()

    val frequencies = List(22) { i ->
        data.sliceArray(36 + i * 4 until 40 + i * 4).toFloatLe()
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
    val data = ByteArray(PRESET_PAYLOAD_SIZE)

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
        System.arraycopy(preset.gains[i].toLeBytes(), 0, data, 124 + i * 4, 4)
    }

    System.arraycopy(preset.skew.toLeBytes(), 0, data, 164, 4)
    System.arraycopy(preset.brightness.toLeBytes(), 0, data, 168, 4)

    data[172] = preset.displayMode.toByte()
    data[173] = preset.animationMode.toByte()

    return data
}