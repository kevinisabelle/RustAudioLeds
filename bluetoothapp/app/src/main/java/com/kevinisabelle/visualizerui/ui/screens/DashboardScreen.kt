package com.kevinisabelle.visualizerui.ui.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Pattern
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.icons.filled.Tune
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Slider
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.collectAsState
import androidx.compose.runtime.getValue
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import androidx.navigation.NavController
import com.kevinisabelle.visualizerui.ble.BleVisualizerRepository
import com.kevinisabelle.visualizerui.ui.components.LedPreview
import dagger.hilt.android.lifecycle.HiltViewModel
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch
import javax.inject.Inject

/** ****************************
 * Dashboard composable & VM
 * **************************** */

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun DashboardScreen(
    navController: NavController,
    viewModel: DashboardViewModel = androidx.hilt.navigation.compose.hiltViewModel()
) {
    val ui by viewModel.ui.collectAsState()

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("LED Visualizer") },
                actions = {
                    IconButton(onClick = { navController.navigate("parameters") }) {
                        Icon(Icons.Default.Tune, contentDescription = "Parameters")
                    }
                    IconButton(onClick = { navController.navigate("presets") }) {
                        Icon(Icons.Default.Pattern, contentDescription = "More")
                    }
                    IconButton(onClick = { viewModel.refresh() }) {
                        Icon(Icons.Default.Refresh, contentDescription = "Refresh")
                    }
                }
            )
        }
    ) { inner ->
        Column(
            Modifier
                .fillMaxWidth()
                .padding(inner)
        ) {

            if (ui.ledColors.isEmpty()) {
                Text(
                    text = "Waiting for data…",
                    style = MaterialTheme.typography.bodyMedium,
                    textAlign = TextAlign.Center
                )
            } else {
                LedPreview(
                    colors = ui.ledColors,
                    columns = 22,
                    rows = 12,
                    modifier = Modifier
                        .fillMaxWidth()
                        .aspectRatio(22f / 12f, matchHeightConstraintsFirst = true)
                )

                Row(
                    modifier = Modifier.padding(5.dp),
                ) {
                    Text("Gain", modifier = Modifier.weight(1f))
                    Slider(
                        value = ui.gain,
                        onValueChange = { viewModel.setGain(it) },
                        valueRange = 1f..25f,
                        modifier = Modifier.weight(3f)
                    )
                }
            }
        }
    }
}

/** ****************************
 * ViewModel
 * **************************** */

@HiltViewModel
class DashboardViewModel @Inject constructor(
    private val repo: BleVisualizerRepository
) : ViewModel() {

    data class Ui(
        val ledColors: List<Color> = emptyList(),
        val running: Boolean = true,
        val gain: Float = 1f
    )

    private val _ui = MutableStateFlow(Ui())
    val ui: StateFlow<Ui> = _ui

    fun refresh() = refreshLedColors()

    private fun refreshLedColors() = viewModelScope.launch {
        val rgbList = repo.getLedColors()
        _ui.update { it.copy(ledColors = rgbList.map { rgb -> Color(rgb or 0xFF000000.toInt()) }) }
        // println("LED colors updated: ${_ui.value.ledColors.size} LEDs")
    }

    fun toggleRunning() {
        viewModelScope.launch { repo.setRunning(!_ui.value.running) }
        _ui.update { it.copy(running = !it.running) }
    }

    fun setGain(b: Float) = viewModelScope.launch {
        repo.setGain(b)
        _ui.update { it.copy(gain = b) }
    }
}
