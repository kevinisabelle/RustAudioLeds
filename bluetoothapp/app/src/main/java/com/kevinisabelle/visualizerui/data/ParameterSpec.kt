package com.kevinisabelle.visualizerui.data

import com.kevinisabelle.visualizerui.data.uuid.visualizerUuid
import java.util.UUID

/**
 * Sealed hierarchy that describes *every* visualizer setting.
 * Each subtype knows its BLE UUID and Kotlin payload type `T`.
 */
sealed interface ParameterSpec<T : Any> {
    val uuid: UUID

    // ---------- concrete entries ----------
    object SmoothSize : ParameterSpec<UShort>   { override val uuid = visualizerUuid(0x0001) }
    object Gain       : ParameterSpec<Float>    { override val uuid = visualizerUuid(0x0002) }
    object Fps        : ParameterSpec<UShort>   { override val uuid = visualizerUuid(0x0003) }

    data class Color(val index: Int) : ParameterSpec<Rgb888> {
        init  { require(index in 1..3) { "Color index must be 1‒3" } }
        override val uuid = visualizerUuid(0x0003 + index)
    }

    object FftSize    : ParameterSpec<UShort>   { override val uuid = visualizerUuid(0x0007) }
    object Frequencies: ParameterSpec<FloatArray> {
        override val uuid = visualizerUuid(0x0008)
        const val COUNT = 22
    }
    object Gains      : ParameterSpec<FloatArray> {
        override val uuid = visualizerUuid(0x0009)
        const val COUNT = 22
    }
    object Skew       : ParameterSpec<Float>    { override val uuid = visualizerUuid(0x000A) }
    object Brightness : ParameterSpec<Float>    { override val uuid = visualizerUuid(0x000B) }
    object Display    : ParameterSpec<DisplayMode> { override val uuid = visualizerUuid(0x000C) }
    object Animation  : ParameterSpec<AnimationMode> { override val uuid = visualizerUuid(0x000D) }
    object LedCount   : ParameterSpec<UShort>   { override val uuid = visualizerUuid(0x000E) }
    object LedsBuffer:  ParameterSpec<ByteArray> {
        override val uuid = visualizerUuid(0x000F)
        const val SIZE = 500 // 264 × RGB888
    }
    object LedsBuffer2: ParameterSpec<ByteArray> {
        override val uuid = visualizerUuid(0x0010)
        const val SIZE = 792 - 500 // 264 × RGB888
    }
    object PresetList: ParameterSpec<ByteArray> {
        override val uuid = visualizerUuid(0x0011)
        const val SIZE = 24 * 17 // 24 × {id: u8, name[16]: UTF-8}
    }

    object PresetSelectIndex: ParameterSpec<UByte> {
        override val uuid = visualizerUuid(0x0012)
    }

    object PresetRead: ParameterSpec<ByteArray> {
        override val uuid = visualizerUuid(0x0013)
        const val SIZE = 222 // 222 B
    }

    object PresetSave: ParameterSpec<ByteArray> {
        override val uuid = visualizerUuid(0x0014)
        const val SIZE = 226 // 226 B
    }

    object PresetActivate: ParameterSpec<UByte> {
        override val uuid = visualizerUuid(0x0015)
    }

    object PresetDelete: ParameterSpec<UByte> {
        override val uuid = visualizerUuid(0x0016)
    }

    object PresetReadActivatedIndex: ParameterSpec<UByte> {
        override val uuid = visualizerUuid(0x0017)
    }

    object SettingsAsPreset: ParameterSpec<ByteArray> {
        override val uuid = visualizerUuid(0x0018)
    }
}