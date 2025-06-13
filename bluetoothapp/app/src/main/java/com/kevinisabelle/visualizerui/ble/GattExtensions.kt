package com.kevinisabelle.visualizerui.ble

import com.kevinisabelle.visualizerui.data.AnimationMode
import com.kevinisabelle.visualizerui.data.DisplayMode
import com.kevinisabelle.visualizerui.data.ParameterSpec
import com.kevinisabelle.visualizerui.data.Rgb888
import java.nio.ByteBuffer
import java.nio.ByteOrder

/* ---------- Primitive helpers ---------- */

private fun leBuffer(size: Int): ByteBuffer =
    ByteBuffer.allocate(size).order(ByteOrder.LITTLE_ENDIAN)
/* ushort (2 B) */
fun UShort.toLeBytes(): ByteArray = leBuffer(2).putShort(toShort()).array()
fun ByteArray.toUShort(): UShort = ByteBuffer.wrap(this).order(ByteOrder.LITTLE_ENDIAN).short.toUShort()

/* float32 (4 B) */
fun Float.toLeBytes(): ByteArray = leBuffer(4).putFloat(this).array()
fun ByteArray.toFloatLe(): Float = ByteBuffer.wrap(this).order(ByteOrder.LITTLE_ENDIAN).float

/* float32 array (N×4 B) */
fun FloatArray.toLeBytes(): ByteArray = leBuffer(size * 4).apply {
    forEach { putFloat(it) }
}.array()

fun ByteArray.toFloatArray(count: Int): FloatArray = FloatArray(count).apply {
    val buf = ByteBuffer.wrap(this@toFloatArray).order(ByteOrder.LITTLE_ENDIAN)
    repeat(count) { idx -> this[idx] = buf.float }
}

/* RGB888 (3 B) */
fun Rgb888.toBytes(): ByteArray = byteArrayOf(r.toByte(), g.toByte(), b.toByte())
fun ByteArray.toRgb888(): Rgb888 = Rgb888(get(0).toUByte(), get(1).toUByte(), get(2).toUByte())

/* enums packed in a single byte */
fun DisplayMode.toByte(): Byte = code.toByte()
fun Byte.toDisplayMode(): DisplayMode = DisplayMode.from(toUByte())

fun AnimationMode.toByte(): Byte = code.toByte()
fun Byte.toAnimationMode(): AnimationMode = AnimationMode.from(toUByte())

/* ---------- Top-level encode/decode ---------- */

fun <T : Any> encode(spec: ParameterSpec<T>, value: T): ByteArray = when (spec) {
    ParameterSpec.SmoothSize,
    ParameterSpec.Fps,
    ParameterSpec.FftSize          -> (value as UShort).toLeBytes()

    ParameterSpec.Gain,
    ParameterSpec.Skew,
    ParameterSpec.Brightness       -> (value as Float).toLeBytes()

    is ParameterSpec.Color         -> (value as Rgb888).toBytes()

    ParameterSpec.Frequencies      -> (value as FloatArray).toLeBytes()
    ParameterSpec.Gains            -> (value as FloatArray).toLeBytes()

    ParameterSpec.Display          -> byteArrayOf((value as DisplayMode).toByte())
    ParameterSpec.Animation        -> byteArrayOf((value as AnimationMode).toByte())
    ParameterSpec.LedCount         -> (value as UShort).toLeBytes()
    ParameterSpec.LedsBuffer       -> (value as ByteArray)
    ParameterSpec.LedsBuffer2      -> (value as ByteArray)
    ParameterSpec.PresetList       -> (value as ByteArray)
    ParameterSpec.PresetSelectIndex -> byteArrayOf((value as UByte).toByte())
    ParameterSpec.PresetRead       -> (value as ByteArray)
    ParameterSpec.PresetSave       -> (value as ByteArray)
    ParameterSpec.PresetActivate   -> byteArrayOf((value as UByte).toByte())
    ParameterSpec.PresetDelete     -> byteArrayOf((value as UByte).toByte())
    ParameterSpec.PresetReadActivatedIndex -> byteArrayOf((value as UByte).toByte())

}

@Suppress("UNCHECKED_CAST")
fun <T : Any> decode(spec: ParameterSpec<T>, bytes: ByteArray): T = when (spec) {
    ParameterSpec.SmoothSize,
    ParameterSpec.Fps,
    ParameterSpec.FftSize          -> bytes.toUShort()    as T

    ParameterSpec.Gain,
    ParameterSpec.Skew,
    ParameterSpec.Brightness       -> bytes.toFloatLe()   as T

    is ParameterSpec.Color         -> bytes.toRgb888()    as T

    ParameterSpec.Frequencies      -> bytes.toFloatArray(ParameterSpec.Frequencies.COUNT) as T
    ParameterSpec.Gains            -> bytes.toFloatArray(ParameterSpec.Gains.COUNT)       as T

    ParameterSpec.Display          -> bytes[0].toDisplayMode()    as T
    ParameterSpec.Animation        -> bytes[0].toAnimationMode()  as T
    ParameterSpec.LedCount         -> bytes.toUShort()    as T
    ParameterSpec.LedsBuffer       -> bytes                as T
    ParameterSpec.LedsBuffer2      -> bytes                as T
    ParameterSpec.PresetList       -> bytes                as T
    ParameterSpec.PresetSelectIndex -> bytes[0].toUByte() as T
    ParameterSpec.PresetRead       -> bytes                as T
    ParameterSpec.PresetSave       -> bytes                as T
    ParameterSpec.PresetActivate   -> bytes[0].toUByte()  as T
    ParameterSpec.PresetDelete     -> bytes[0].toUByte()  as T
    ParameterSpec.PresetReadActivatedIndex -> bytes[0].toUByte() as T
}
