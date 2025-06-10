package com.kevinisabelle.visualizerui.ble

/** Results of a single scan attempt. */
sealed interface ScanResult {
    data class Success(val devices: List<ScannedDevice>) : ScanResult
    data class Error(
        val message: String,
        val actionLabel: String = "",
        val recoveryAction: (suspend ((/* retry */) -> Unit) -> Unit)? = null,
    ) : ScanResult
}