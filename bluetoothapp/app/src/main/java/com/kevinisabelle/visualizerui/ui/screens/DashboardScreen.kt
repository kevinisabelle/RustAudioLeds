package com.kevinisabelle.visualizerui.ui.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.aspectRatio
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.GraphicEq
import androidx.compose.material.icons.filled.Pattern
import androidx.compose.material.icons.filled.PlayCircle
import androidx.compose.material.icons.filled.Refresh
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material.icons.filled.StopCircle
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
import com.airbnb.lottie.compose.LottieAnimation
import com.airbnb.lottie.compose.LottieCompositionSpec
import com.airbnb.lottie.compose.LottieConstants
import com.airbnb.lottie.compose.animateLottieCompositionAsState
import com.airbnb.lottie.compose.rememberLottieComposition
import com.kevinisabelle.visualizerui.R
import com.kevinisabelle.visualizerui.ble.BleVisualizerRepository
import com.kevinisabelle.visualizerui.data.AnimationMode
import com.kevinisabelle.visualizerui.data.DisplayMode
import com.kevinisabelle.visualizerui.data.Rgb888
import com.kevinisabelle.visualizerui.services.Settings
import com.kevinisabelle.visualizerui.ui.components.DeviceSettings
import com.kevinisabelle.visualizerui.ui.components.LedPreview
import com.kevinisabelle.visualizerui.ui.components.TitleRow
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

                modifier = Modifier.padding(horizontal = 8.dp, vertical = 4.dp).fillMaxWidth()
            )
            {
                Row(
                    horizontalArrangement = androidx.compose.foundation.layout.Arrangement.SpaceEvenly,
                    verticalAlignment = androidx.compose.ui.Alignment.CenterVertically,
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(8.dp)
                ) {
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
        }

    ) { inner ->
        Column(
            Modifier
                .fillMaxWidth()
                .padding(inner)
        ) {

            when (ui.currentPanel) {
                "Visualizer" -> {
                    Column(
                        modifier = Modifier.padding(8.dp),
                        verticalArrangement = androidx.compose.foundation.layout.Arrangement.spacedBy(8.dp)
                    )
                    {
                        Row(
                            verticalAlignment = androidx.compose.ui.Alignment.CenterVertically,
                            horizontalArrangement = androidx.compose.foundation.layout.Arrangement.Center
                        )
                        {
                            IconButton(onClick = { viewModel.refresh() }) {
                                Icon(
                                    Icons.Default.Refresh,
                                    contentDescription = "Refresh LED Colors"
                                )
                            }
                            Text(
                                text = "Visualizer",
                                style = MaterialTheme.typography.titleLarge,
                                textAlign = TextAlign.Center,
                                modifier = Modifier.weight(1f)
                            )
                            IconButton(
                                onClick = {
                                    viewModel.setPreviewAnimation(!viewModel.getPreviewAnimation())
                                },
                                modifier = Modifier.padding(start = 8.dp)
                            ) {
                                Icon(
                                    imageVector = viewModel.getPreviewAnimation()
                                        .let { if (it) Icons.Default.StopCircle else Icons.Default.PlayCircle },
                                    contentDescription = "Start Animation",
                                )
                            }
                        }
                        TitleRow(title = "LED Preview")

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

                "Settings" -> {
                    if (ui.settings == null) {

                        Column(
                            modifier = Modifier.padding(8.dp),
                            verticalArrangement = androidx.compose.foundation.layout.Arrangement.spacedBy(8.dp)
                        ) {
                            Row(
                                verticalAlignment = androidx.compose.ui.Alignment.CenterVertically,
                                horizontalArrangement = androidx.compose.foundation.layout.Arrangement.Center
                            )
                            {
                                IconButton(
                                    onClick = { viewModel.getSettings() },
                                    modifier = Modifier.padding(start = 8.dp)
                                ) {
                                    Icon(
                                        imageVector = Icons.Default.Refresh,
                                        contentDescription = "Reload settings"
                                    )
                                }
                                Text(
                                    text = "Settings",
                                    style = MaterialTheme.typography.titleLarge,
                                    textAlign = TextAlign.Center,
                                    modifier = Modifier.weight(1f)
                                )
                            }
                            Column(
                                horizontalAlignment = androidx.compose.ui.Alignment.CenterHorizontally,
                                modifier = Modifier
                                    .fillMaxWidth()
                                    .padding(16.dp)
                            ){
                                if (ui.loading) {
                                    val composition by rememberLottieComposition(LottieCompositionSpec.RawRes(R.raw.connecting))
                                    val progress by animateLottieCompositionAsState(composition, iterations = LottieConstants.IterateForever)

                                    LottieAnimation(
                                        composition = composition,
                                        progress = { progress },
                                        modifier = Modifier.size(160.dp)
                                    )
                                } else {
                                    Button(
                                        onClick = { viewModel.getSettings() },
                                        modifier = Modifier.padding(16.dp)
                                    ) {
                                        Text("Load Settings")
                                    }
                                }
                            }

                        }

                        return@Column
                    }

                    DeviceSettings(
                        settings = ui.settings ?: Settings(),
                        onSetGain = { gain ->
                            viewModel.setGain(gain)
                        },
                        onSetFrequencyGain = { freq, gain ->
                            viewModel.setGains(ui.settings?.gains?.mapIndexed { index, _ ->
                                if (index == freq) gain else ui.settings?.gains?.get(index) ?: 0f
                            } ?: emptyList())
                        },
                        onSetSkew = { skew ->
                            viewModel.setSkew(skew)
                        },
                        onSetSmoothSize = { smoothSize ->
                            viewModel.setSmoothSize(smoothSize)
                        },
                        onSetFps = { fps ->
                            viewModel.setFps(fps)
                        },
                        onSetBrightness = { brightness ->
                            viewModel.setBrightness(brightness)
                        },
                        onSetColor1 = { color ->
                            viewModel.setColor1(color)
                        },
                        onSetColor2 = { color ->
                            viewModel.setColor2(color)
                        },
                        onSetColor3 = { color ->
                            viewModel.setColor3(color)
                        },
                        onSetFftSize = { size ->
                            viewModel.setFFTSize(size)
                        },
                        onSetDisplayMode = { mode ->
                            viewModel.setDisplayMode(mode)
                        },
                        onSetAnimationMode = { mode ->
                            viewModel.setAnimationMode(mode)
                        },
                        onRefreshClick = { viewModel.getSettings() },
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
        val currentPanel: String = "Visualizer",
        val loading : Boolean = false,
        val previewAnimation: Boolean = false
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
        _ui.update { it.copy(settings = null, loading = true) } // Reset settings before fetching
        val settings = repo.getSettings()
        _ui.update { it.copy(settings = settings, loading = false) }
    }

    fun setGain(b: Float) = viewModelScope.launch {
        repo.setGain(b)
        if (_ui.value.settings != null) {
            _ui.update { it.copy(settings = it.settings?.copy(gain = b)) }
        }
    }

    fun setFFTSize(size: Int) = viewModelScope.launch {
        repo.setFftSize(size.toUShort())
        if (_ui.value.settings != null) {
            _ui.update { it.copy(settings = it.settings?.copy(fftSize = size.toUShort())) }
        }
    }

    fun setDisplayMode(mode: DisplayMode) = viewModelScope.launch {
        repo.setDisplayMode(mode)
        if (_ui.value.settings != null) {
            _ui.update { it.copy(settings = it.settings?.copy(displayMode = mode)) }
        }
    }

    fun setAnimationMode(mode: AnimationMode) = viewModelScope.launch {
        repo.setAnimationMode(mode)
        if (_ui.value.settings != null) {
            _ui.update { it.copy(settings = it.settings?.copy(animationMode = mode)) }
        }
    }

    fun setSmoothSize(size: Int) = viewModelScope.launch {
        repo.setSmoothSize(size)
        if (_ui.value.settings != null) {
            _ui.update { it.copy(settings = it.settings?.copy(smoothSize = size)) }
        }
    }

    fun setSkew(skew: Float) = viewModelScope.launch {
        repo.setSkew(skew)
        if (_ui.value.settings != null) {
            _ui.update { it.copy(settings = it.settings?.copy(skew = skew)) }
        }
    }

    fun setBrightness(brightness: Float) = viewModelScope.launch {
        repo.setBrightness(brightness)
        if (_ui.value.settings != null) {
            _ui.update { it.copy(settings = it.settings?.copy(brightness = brightness)) }
        }
    }

    fun setColor1(color: Rgb888) = viewModelScope.launch {
        repo.setColor1(color)
        if (_ui.value.settings != null) {
            _ui.update { it.copy(settings = it.settings?.copy(color1 = color)) }
        }
    }

    fun setColor2(color: Rgb888) = viewModelScope.launch {
        repo.setColor2(color)
        if (_ui.value.settings != null) {
            _ui.update { it.copy(settings = it.settings?.copy(color2 = color)) }
        }
    }

    fun setColor3(color: Rgb888) = viewModelScope.launch {
        repo.setColor3(color)
        if (_ui.value.settings != null) {
            _ui.update { it.copy(settings = it.settings?.copy(color3 = color)) }
        }
    }

    fun setGains(gains: List<Float>) = viewModelScope.launch {
        repo.setGains(gains)
        if (_ui.value.settings != null) {
            _ui.update { it.copy(settings = it.settings?.copy(gains = gains)) }
        }
    }

    fun setFps(fps: Long) = viewModelScope.launch {
        repo.setFps(fps)
        if (_ui.value.settings != null) {
            _ui.update { it.copy(settings = it.settings?.copy(fps = fps)) }
        }
    }

    fun setPreviewAnimation(enabled: Boolean) = viewModelScope.launch {
        _ui.update { it.copy(previewAnimation = enabled) }
        if (enabled) {
            // Launch a coroutine to start the animation that will run in the background using the startAnimation function
            startAnimation()

        }
    }

    fun getPreviewAnimation(): Boolean {
        return _ui.value.previewAnimation
    }

    // Coroutine to handle the animation logic
    // This could be a long-running task that updates the LED colors periodically
    fun startAnimation() = viewModelScope.launch {
        while (_ui.value.previewAnimation) {
            refreshLedColors().join()
            kotlinx.coroutines.delay(1000 / 30) // Adjust delay based on FPS
        }
    }
}
