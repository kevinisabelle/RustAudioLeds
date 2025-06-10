package com.kevinisabelle.visualizerui.ui.screens

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.MoreVert
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.navigation.NavController
import androidx.compose.material.icons.filled.PlayArrow
import androidx.compose.material.icons.filled.Pause
import androidx.compose.material.icons.filled.Tune
import androidx.compose.ui.text.style.TextAlign
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.kevinisabelle.visualizerui.ble.BleVisualizerRepository
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.collectLatest
import kotlinx.coroutines.flow.update
import kotlinx.coroutines.launch

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
                        Icon(Icons.Default.MoreVert, contentDescription = "More")
                    }
                }
            )
        },
        bottomBar = {
            QuickControls(
                running = ui.running,
                gain = ui.gain,
                onToggleRunning = { viewModel.toggleRunning() },
                onGainChange = { viewModel.setGain(it) }
            )
        }
    ) { inner ->
        Box(
            Modifier
                .fillMaxSize()
                .padding(inner),
            contentAlignment = Alignment.Center
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
                        .aspectRatio(22f / 12f)
                )
            }
        }
    }
}

/** Draws the LED matrix as coloured squares. */
@Composable
private fun LedPreview(
    colors: List<Color>,
    columns: Int,
    rows: Int,
    modifier: Modifier = Modifier
) {
    Canvas(modifier) {
        if (colors.isEmpty()) return@Canvas
        val cellW = size.width / columns
        val cellH = size.height / rows
        for (y in 0 until rows) {
            for (x in 0 until columns) {
                val idx = y * columns + x
                if (idx < colors.size) {
                    drawRect(
                        color = colors[idx],
                        topLeft = androidx.compose.ui.geometry.Offset(x * cellW, y * cellH),
                        size = Size(cellW, cellH)
                    )
                }
            }
        }
    }
}

@Composable
private fun QuickControls(
    running: Boolean,
    gain: Float,
    onToggleRunning: () -> Unit,
    onGainChange: (Float) -> Unit,
) {
    Column(Modifier.fillMaxWidth()) {
        Row(
            Modifier
                .fillMaxWidth()
                .padding(16.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(16.dp)
        ) {
            IconButton(onClick = onToggleRunning) {
                Icon(
                    if (running) Icons.Default.Pause else Icons.Default.PlayArrow,
                    contentDescription = if (running) "Pause" else "Play"
                )
            }
            Text("Brightness", modifier = Modifier.weight(1f))
            Slider(
                value = gain,
                onValueChange = onGainChange,
                valueRange = 0f..1f,
                modifier = Modifier.weight(3f)
            )
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

    init {
        viewModelScope.launch {
            repo.ledBufferFlow().collectLatest { rgbList ->
                _ui.update { it.copy(ledColors = rgbList.map { rgb -> Color(rgb) }) }
            }
        }
    }

    fun toggleRunning() {
        viewModelScope.launch { repo.setRunning(!_ui.value.running) }
        _ui.update { it.copy(running = !it.running) }
    }

    fun setGain(b: Float) {
        viewModelScope.launch { repo.setGain(b) }
        _ui.update { it.copy(gain = b) }
    }
}
