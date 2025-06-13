package com.kevinisabelle.visualizerui.data

import com.kevinisabelle.visualizerui.services.Settings
import java.io.ByteArrayOutputStream

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

fun encodePreset(preset: Preset): ByteArray {
    val csv = encodePresetCsv(preset)

    val bytes = csv.toByteArray()
    val zippedBytes = java.util.zip.Deflater().apply {
        setInput(bytes)
        finish()
    }

    val outputStream = ByteArrayOutputStream()
    while (!zippedBytes.finished()) {
        val buffer = ByteArray(1024)
        val count = zippedBytes.deflate(buffer)
        if (count > 0) {
            outputStream.write(buffer, 0, count)
        }
    }

    val compressedBytes = outputStream.toByteArray()
    return compressedBytes
}

fun decodePreset(data: ByteArray): Preset {
    val inflater = java.util.zip.Inflater()
    inflater.setInput(data)

    val outputStream = ByteArrayOutputStream()
    val buffer = ByteArray(1024)
    while (!inflater.finished()) {
        val count = inflater.inflate(buffer)
        if (count > 0) {
            outputStream.write(buffer, 0, count)
        }
    }

    val decompressedBytes = outputStream.toByteArray()
    val csv = decompressedBytes.decodeToString()

    return decodePresetCsv(csv)
}

fun encodePresetCsv(preset: Preset): String {
    return "${preset.index}," +
            "${preset.name.replace(',', ' ')}," + // Escape commas in name
            "${preset.smoothSize}," +
            "${preset.gain}," +
            "${preset.fps}," +
            "${preset.color1.toHex()}," +
            "${preset.color2.toHex()}," +
            "${preset.color3.toHex()}," +
            "${preset.fftSize}," +
            "[" +
            preset.frequencies.joinToString("|") + "]," +
            "[" +
            preset.gains.joinToString("|") + "]," +
            "${preset.skew}," +
            "${preset.brightness}," +
            "${preset.displayMode.code}," +
            "${preset.animationMode.code}"
}

fun decodePresetCsv(csv: String): Preset {
    val parts = csv.split(',')
    require(parts.size == 15) { "Invalid preset CSV format" }

    val index = parts[0].toUByte()
    val name = parts[1].trim()
    require(name.length <= Preset.NAME_MAX_LENGTH) { "Preset name exceeds maximum length" }

    val smoothSize = parts[2].toUShort()
    val gain = parts[3].toFloat()
    val fps = parts[4].toUShort()

    val color1 = Rgb888.fromHex(parts[5])
    val color2 = Rgb888.fromHex(parts[6])
    val color3 = Rgb888.fromHex(parts[7])

    val fftSize = parts[8].toUShort()

    val frequencies = parts[9].removeSurrounding("[", "]").split('|').map { it.toFloat() }
    require(frequencies.size == 22) { "Frequencies must contain exactly 22 values" }

    val gains = parts[10].removeSurrounding("[", "]").split('|').map { it.toFloat() }
    require(gains.size == 22) { "Gains must contain exactly 22 values" }

    val skew = parts[11].toFloat()
    val brightness = parts[12].toFloat()

    val displayMode = DisplayMode.from(parts[13].toUByte())
    val animationMode = AnimationMode.from(parts[14].toUByte())

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
