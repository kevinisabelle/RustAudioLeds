package com.kevinisabelle.visualizerui.ble

/** UI mode/state for the ScanScreen. */
sealed interface ScanUi {
    data object Scanning : ScanUi
    data object DeviceList : ScanUi
    data object Error : ScanUi
}