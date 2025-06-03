package com.kevinisabelle.visualizerui

import android.Manifest
import android.os.Build
import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material.icons.Icons
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.unit.dp
import androidx.core.view.WindowCompat
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.google.accompanist.permissions.ExperimentalPermissionsApi
import com.google.accompanist.permissions.MultiplePermissionsState
import com.google.accompanist.permissions.rememberMultiplePermissionsState
import dagger.hilt.android.AndroidEntryPoint

/**
 * Single‑activity entry point hosting the whole Compose navigation graph.
 */
@AndroidEntryPoint
class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        WindowCompat.setDecorFitsSystemWindows(window, false)

        setContent {
            VisualizerTheme {
                val navController = rememberNavController()
                VisualizerNavHost(navController = navController)
            }
        }
    }
}

/**
 * Root navigation graph.
 */
@Composable
fun VisualizerNavHost(navController: NavHostController) {
    NavHost(
        navController = navController,
        startDestination = Routes.SPLASH,
        modifier = Modifier.fillMaxSize()
    ) {
        composable(Routes.SPLASH) {
            SplashPermScreen(
                onPermissionsGranted = {
                    navController.navigate(Routes.SCAN) {
                        popUpTo(Routes.SPLASH) { inclusive = true }
                    }
                }
            )
        }
        composable(Routes.SCAN) { /* TODO ScanScreen() */ }
        composable(Routes.CONNECTING) { /* TODO ConnectingScreen() */ }
        composable(Routes.DASHBOARD) { /* TODO DashboardScreen() */ }
        composable(Routes.PRESETS) { /* TODO PresetsScreen() */ }
        composable(Routes.PARAMETERS) { /* TODO ParametersScreen() */ }
        composable(Routes.SETTINGS) { /* TODO SettingsScreen() */ }
        composable(Routes.ABOUT) { /* TODO AboutScreen() */ }
    }
}

object Routes {
    const val SPLASH = "splash/perm"
    const val SCAN = "scan"
    const val CONNECTING = "connecting"
    const val DASHBOARD = "dashboard"
    const val PRESETS = "presets"
    const val PARAMETERS = "parameters"
    const val SETTINGS = "settings"
    const val ABOUT = "about"
}

/**
 * First‑run “splash” that acquires runtime permissions, then hops to Scan.
 */
@OptIn(ExperimentalPermissionsApi::class)
@Composable
fun SplashPermScreen(onPermissionsGranted: () -> Unit) {
    val requiredPerms = remember {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) listOf(
            Manifest.permission.BLUETOOTH_SCAN,
            Manifest.permission.BLUETOOTH_CONNECT,
            Manifest.permission.ACCESS_FINE_LOCATION,
            Manifest.permission.POST_NOTIFICATIONS,
        ) else listOf(
            Manifest.permission.BLUETOOTH,
            Manifest.permission.BLUETOOTH_ADMIN,
            Manifest.permission.ACCESS_FINE_LOCATION,
        )
    }

    val permState: MultiplePermissionsState = rememberMultiplePermissionsState(requiredPerms)

    // When all perms granted, jump ahead.
    LaunchedEffect(permState.allPermissionsGranted) {
        if (permState.allPermissionsGranted) onPermissionsGranted()
    }

    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(MaterialTheme.colorScheme.primary),
        contentAlignment = Alignment.Center
    ) {
        Column(horizontalAlignment = Alignment.CenterHorizontally) {
            Icon(
                imageVector = Icons.Default.Bluetooth,
                contentDescription = null,
                tint = MaterialTheme.colorScheme.onPrimary,
                modifier = Modifier.size(96.dp)
            )
            Spacer(Modifier.height(24.dp))

            if (!permState.allPermissionsGranted) {
                Text(
                    "We need Bluetooth & Location permissions to control your LEDs.",
                    color = MaterialTheme.colorScheme.onPrimary
                )
                Spacer(Modifier.height(16.dp))
                Button(onClick = { permState.launchMultiplePermissionRequest() }) {
                    Text("Grant permissions")
                }
            } else {
                CircularProgressIndicator(color = MaterialTheme.colorScheme.onPrimary)
            }
        }
    }
}

/**
 * Placeholder theme – adopt your Material 3 color‑scheme later.
 */
@Composable
fun VisualizerTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = lightColorScheme(
            primary = Color(0xFF0066FF),
            onPrimary = Color.White
        ),
        content = content
    )
}
