package com.kevinisabelle.visualizerui.ble

sealed interface ConnectionState {
    object Disconnected : ConnectionState
    object Connecting   : ConnectionState
    data class Connected(val deviceAddress: String) : ConnectionState
    data class Failed(val reason: String) : ConnectionState
}