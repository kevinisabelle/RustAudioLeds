package com.kevinisabelle.visualizerui.ui.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.GraphicEq
import androidx.compose.material.icons.filled.Pattern
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material3.Button
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.Icon
import androidx.compose.material3.IconButton
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.NavigationBar
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
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
import com.kevinisabelle.visualizerui.services.Settings
import com.kevinisabelle.visualizerui.ui.components.DeviceSettings
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
        bottomBar = {
            NavigationBar(
               // Center the content
                modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp)
            )
            {
                IconButton(
                    onClick = {
                        viewModel.gotoPanel("Visualizer")
                    },
                ) {
                    Icon(
                        imageVector = Icons.Default.GraphicEq,
                        contentDescription = "Viewer",
                    )
                }
                IconButton(
                    onClick = {
                        viewModel.gotoPanel("Settings")
                    },
                ) {
                    Icon(
                        imageVector = Icons.Default.Settings,
                        contentDescription = "Settings",
                    )
                }
                IconButton(
                    onClick = {
                        viewModel.gotoPanel("Presets")
                    }
                ) {
                    Icon(Icons.Default.Pattern, contentDescription = "Presets")
                }
            }
        }

    ) { inner ->
        Column(
            Modifier
                .fillMaxWidth()
                .padding(inner)
        ) {

            when (ui.currentPanel) {
                "Visualizer" -> {

                    Row(
                        modifier = Modifier
                            .fillMaxWidth()
                            .padding(8.dp),
                        horizontalArrangement = androidx.compose.foundation.layout.Arrangement.SpaceBetween
                    ) {
                        IconButton(onClick = { viewModel.refresh() }) {
                            Icon(Icons.Default.Refresh, contentDescription = "Refresh LED Colors")
                        }
                        Text(
                            text = "LED Preview",
                            style = MaterialTheme.typography.titleMedium,
                            textAlign = TextAlign.Center
                        )
                    }

                    LedPreview(
                        colors = ui.ledColors,
                        columns = 22,
                        rows = 12,
                        modifier = Modifier
                            .fillMaxWidth()
                            .aspectRatio(22f / 12f, matchHeightConstraintsFirst = true)
                    )
                }

                "Settings" -> {
                    if (ui.settings == null) {
                        Text(
                            text = "No settings available",
                            style = MaterialTheme.typography.bodyMedium,
                            textAlign = TextAlign.Center
                        )
                        Button(
                            onClick = { viewModel.getSettings() },
                            modifier = Modifier.padding(16.dp)
                        ) {
                            Text("Load Settings")
                        }
                        return@Column
                    }

                    DeviceSettings(
                        settings = ui.settings ?: Settings(),
                        onSetGain = { },
                        onSetFrequencyGain = { freq, gain -> Unit },
                        onSetSkew = { },
                        onSetSmoothSize = { },
                    )
                }

                "Presets" -> {
                    // Presets UI would go here
                    Text(
                        text = "Presets feature is not implemented yet.",
                        style = MaterialTheme.typography.bodyMedium,
                        textAlign = TextAlign.Center
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
    private val repo: BleVisualizerRepository,
) : ViewModel() {

    data class Ui(
        val ledColors: List<Color> = emptyList(),
        val settings: Settings? = null,
        val panels: List<String> = listOf(
            "Visualizer",
            "Settings",
            "Presets"
        ),
        val currentPanel: String = "Visualizer"
    )

    private val _ui = MutableStateFlow(Ui())
    val ui: StateFlow<Ui> = _ui

    fun refresh() = refreshLedColors()

    private fun refreshLedColors() = viewModelScope.launch {
        val rgbList = repo.getLedColors()
        _ui.update { it.copy(ledColors = rgbList.map { rgb -> Color(rgb or 0xFF000000.toInt()) }) }
        // println("LED colors updated: ${_ui.value.ledColors.size} LEDs")
    }

    fun gotoPanel(panel: String) = viewModelScope.launch {
        if (panel in _ui.value.panels) {
            _ui.update { it.copy(currentPanel = panel) }
        }
    }

    fun getSettings() = viewModelScope.launch {
        val settings = repo.getSettings()
        _ui.update { it.copy(settings = settings) }
    }

    fun setGain(b: Float) = viewModelScope.launch {
        repo.setGain(b)
        if (_ui.value.settings != null) {
            _ui.update { it.copy(settings = it.settings?.copy(gain = b)) }
        }
    }
}
