package com.kevinisabelle.visualizerui.ui.screens

import android.bluetooth.BluetoothDevice
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.kevinisabelle.visualizerui.ble.BleVisualizerRepository
import com.kevinisabelle.visualizerui.ble.ScanResult
import com.kevinisabelle.visualizerui.ble.ScanUi
import com.kevinisabelle.visualizerui.ble.ScannedDevice
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import javax.inject.Inject

@HiltViewModel
class ScanViewModel @Inject constructor(
    private val repo: BleVisualizerRepository
) : ViewModel() {

    data class Ui(
        val state: ScanUi = ScanUi.Scanning,
        val devices: List<ScannedDevice> = emptyList(),
        val errorMessage: String = "",
        val actionLabel: String = "",
        val onAction: () -> Unit = {},
        val canRefresh: Boolean = false,
        val navigateToConnecting: BluetoothDevice? = null
    )
    private val _ui = MutableStateFlow(Ui())
    val ui: StateFlow<Ui> = _ui

    init { scan() }

    fun refresh() = scan()

    private fun scan() = viewModelScope.launch {
        _ui.update { it.copy(state = ScanUi.Scanning, devices = emptyList(), canRefresh = false) }
        when (val result = repo.scanOnce()) {           // suspend fun using Flow internally
            is ScanResult.Success -> _ui.update {
                it.copy(
                    state = ScanUi.DeviceList,
                    devices = result.devices,
                    canRefresh = true
                )
            }
            is ScanResult.Error -> _ui.update {
                it.copy(
                    state = ScanUi.Error,
                    errorMessage = result.message,
                    actionLabel = result.actionLabel,
                    onAction = { result.recoveryAction(::scan) },
                    canRefresh = true
                )
            }
        }
    }

    fun connect(d: BluetoothDevice) {
        _ui.update { it.copy(navigateToConnecting = d) }
    }
    fun onNavigationDone() = _ui.update { it.copy(navigateToConnecting = null) }
}