package com.kevinisabelle.visualizerui.ble

/** Results of a connect‑&‑discover sequence. */
sealed interface ConnectResult {
    data class Success(val device: BleVisualizerDevice) : ConnectResult
    data class Error(val message: String) : ConnectResult
    data object Cancelled : ConnectResult
}