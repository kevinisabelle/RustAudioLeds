package com.kevinisabelle.visualizerui.ui.screens


import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothDevice
import android.bluetooth.BluetoothManager
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Close
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.platform.LocalContext
import androidx.compose.ui.unit.dp
import androidx.core.content.ContextCompat.getSystemService
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.navigation.NavController
import com.airbnb.lottie.compose.*
import com.kevinisabelle.visualizerui.R
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.kevinisabelle.visualizerui.ble.BleVisualizerRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import com.kevinisabelle.visualizerui.ble.ConnectResult

/**
 * Composable that shows a Lottie spinner while we connect & discover GATT.
 * Route format: "connecting/{address}" with the BT MAC address as arg.
 */
@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ConnectingScreen(
    navController: NavController,
    address: String,
    viewModel: ConnectingViewModel = hiltViewModel<ConnectingViewModel>()
) {

    val ui by viewModel.ui.collectAsState()

    val context = LocalContext.current
    LaunchedEffect(address, context) {
        // Get the bluetooth manager from hilt
        val bluetoothManager = getSystemService(
            context,
            BluetoothManager::class.java
        )
        val bluetoothAdapter = bluetoothManager?.adapter
        if (bluetoothAdapter != null && BluetoothAdapter.checkBluetoothAddress(address)) {
            try {
                val deviceToConnect: BluetoothDevice = bluetoothAdapter.getRemoteDevice(address)
                viewModel.start(deviceToConnect) // Call with BluetoothDevice object
            } catch (e: IllegalArgumentException) {
                // Handle invalid MAC address format, update UI accordingly via ViewModel
                // e.g., viewModel.updateProgress("Error: Invalid device address.")
            }
        } else {
            // Handle Bluetooth not available or invalid address, update UI accordingly
            // e.g., viewModel.updateProgress("Error: Bluetooth not available or invalid address.")
        }
    }

    // navigate away when done
    LaunchedEffect(ui.done) {
        if (ui.done) navController.navigate("dashboard") {
            popUpTo("scan") { inclusive = false }
        }
    }

    Scaffold(topBar = {
        TopAppBar(
            title = { Text("Connecting…") },
            actions = {
                IconButton(onClick = { viewModel.cancel(); navController.popBackStack() }) {
                    Icon(Icons.Default.Close, contentDescription = "Cancel")
                }
            })
    }) { inner ->
        Box(
            modifier = Modifier
                .fillMaxSize()
                .padding(inner),
            contentAlignment = Alignment.Center
        ) {
            ConnectingContent(ui.progressMessage)
        }
    }
}

@Composable
private fun ConnectingContent(msg: String) {
    val composition by rememberLottieComposition(LottieCompositionSpec.RawRes(R.raw.connecting))
    val progress by animateLottieCompositionAsState(composition, iterations = LottieConstants.IterateForever)

    Column(horizontalAlignment = Alignment.CenterHorizontally) {
        LottieAnimation(
            composition = composition,
            progress = { progress },
            modifier = Modifier.size(160.dp)
        )
        Spacer(Modifier.height(24.dp))
        Text(msg, style = MaterialTheme.typography.bodyLarge)
    }
}

// -------------------- ViewModel --------------------

@HiltViewModel
class ConnectingViewModel @Inject constructor(
    private val repo: BleVisualizerRepository
) : ViewModel() {

    data class UiState(
        val progressMessage: String = "Connecting…",
        val done: Boolean = false
    )

    private val _ui = MutableStateFlow(UiState())
    val ui: StateFlow<UiState> = _ui

    fun start(device: BluetoothDevice) {
        // avoid re‑entry if already in progress
        if (_ui.value.done || _ui.value.progressMessage != "Connecting…") return

        viewModelScope.launch {
            _ui.update { it.copy(progressMessage = "Connecting…") }
            // Call connectAndDiscover with the BluetoothDevice object
            when (val res = repo.connectAndDiscover(device)) {
                is ConnectResult.Success -> _ui.update { it.copy(progressMessage = "Connected!", done = true) }
                is ConnectResult.Error -> _ui.update { it.copy(progressMessage = res.message) } // Assumes res.msg is valid
                is ConnectResult.Cancelled -> _ui.update { it.copy(progressMessage = "Connection cancelled.") } // Added Cancelled branch
                // If ConnectResult has other states, an 'else' branch might be needed.
            }
        }
    }

    fun cancel() {
        repo.cancelConnect()
        // Optionally, update UI state to reflect cancellation initiated by user
        // _ui.update { it.copy(progressMessage = "Cancelling connection...", done = false) }
    }
}