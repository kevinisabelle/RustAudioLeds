package com.kevinisabelle.visualizerui.ui.screens

import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.collectAsState
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.navigation.NavController
import com.kevinisabelle.visualizerui.ble.ScanUi

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
                onConnect = { device -> viewModel.connect(device) },
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