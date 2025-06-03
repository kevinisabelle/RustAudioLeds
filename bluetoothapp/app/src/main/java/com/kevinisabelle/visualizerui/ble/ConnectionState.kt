package com.kevinisabelle.visualizerui.ble

import java.util.UUID

sealed interface ConnectionState {
    object Disconnected : ConnectionState
    object Connecting   : ConnectionState
    data class Connected(val deviceAddress: UUID) : ConnectionState
    data class Failed(val reason: String) : ConnectionState
}