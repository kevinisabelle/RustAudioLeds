package com.kevinisabelle.visualizerui.ui.screens

import android.annotation.SuppressLint
import android.bluetooth.BluetoothDevice
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import androidx.navigation.NavController
import com.kevinisabelle.visualizerui.ble.BleVisualizerRepository
import com.kevinisabelle.visualizerui.ble.ScanResult
import com.kevinisabelle.visualizerui.ble.ScanUi
import com.kevinisabelle.visualizerui.ble.ScannedDevice
import com.kevinisabelle.visualizerui.ui.components.DeviceList
import com.kevinisabelle.visualizerui.ui.components.ErrorCard
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import javax.inject.Inject

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ScanScreen(
    navController: NavController,
    viewModel: ScanViewModel = hiltViewModel()
) {
    val ui by viewModel.ui.collectAsState()

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Select device") },
                actions = {
                    IconButton(
                        onClick = { viewModel.refresh() },
                        enabled = ui.canRefresh
                    ) { Icon(Icons.Default.Refresh, contentDescription = "Refresh") }
                }
            )
        }
    ) { innerPadding ->
        when (ui.state) {
            ScanUi.DeviceList -> DeviceList(
                devices = ui.devices,
                onConnect = { device -> viewModel.connect(device.device) },
                modifier = Modifier.padding(innerPadding)
            )
            ScanUi.Scanning -> Box(
                Modifier
                    .fillMaxSize()
                    .padding(innerPadding),
                contentAlignment = Alignment.Center
            ) { CircularProgressIndicator() }
            ScanUi.Error -> ErrorCard(
                message = ui.errorMessage,
                actionLabel = ui.actionLabel,
                onAction = ui.onAction,
                modifier = Modifier.padding(innerPadding)
            )
        }
    }

    /* Navigation side-effect */
    LaunchedEffect(ui.navigateToConnecting) {
        ui.navigateToConnecting?.let { device ->
            navController.navigate("connecting/${device.address}")
            viewModel.onNavigationDone()
        }
    }
}

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

    @SuppressLint("MissingPermission")
    private fun scan() = viewModelScope.launch {
        _ui.update { it.copy(state = ScanUi.Scanning, devices = emptyList(), canRefresh = false) }
        when (val result = repo.scanOnce()) {           // suspend fun using Flow internally
            is ScanResult.Success -> _ui.update {
                val filteredDevices = result.devices.filter { it.device?.name?.isNotEmpty() == true && it.device.name.startsWith("Led") }
                it.copy(
                    state = ScanUi.DeviceList,
                    devices = filteredDevices,
                    canRefresh = true
                )
            }
            is ScanResult.Error -> _ui.update {
                it.copy(
                    state = ScanUi.Error,
                    errorMessage = result.message,
                    actionLabel = result.actionLabel,
                    onAction = { result.recoveryAction?.let { it1 -> viewModelScope.launch { it1(::scan) } } },
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