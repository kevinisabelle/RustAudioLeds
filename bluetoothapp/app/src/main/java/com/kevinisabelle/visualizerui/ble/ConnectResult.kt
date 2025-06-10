package com.kevinisabelle.visualizerui.ble

import android.bluetooth.BluetoothGatt

/** Results of a connect‑&‑discover sequence. */
sealed interface ConnectResult {
    data class Success(val gatt: BluetoothGatt) : ConnectResult
    data class Error(val message: String) : ConnectResult
    data object Cancelled : ConnectResult
}