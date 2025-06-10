package com.kevinisabelle.visualizerui.ble

import android.bluetooth.BluetoothDevice

/** Wrapper for one BLE advertisement. */
data class ScannedDevice(
    val device: BluetoothDevice,
    val displayName: String,
    val rssi: Int,
)