package com.kevinisabelle.visualizerui

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.core.view.WindowCompat
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import androidx.navigation.NavType
import androidx.navigation.navArgument
import com.kevinisabelle.visualizerui.ui.screens.ConnectingScreen
import com.kevinisabelle.visualizerui.ui.screens.ConnectingViewModel
import com.kevinisabelle.visualizerui.ui.screens.DashboardScreen
import com.kevinisabelle.visualizerui.ui.screens.DashboardViewModel
import com.kevinisabelle.visualizerui.ui.screens.ScanScreen
import com.kevinisabelle.visualizerui.ui.screens.ScanViewModel
import com.kevinisabelle.visualizerui.ui.screens.SplashPermScreen
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
        composable(Routes.SCAN) {
            ScanScreen(
                navController,
                viewModel = hiltViewModel<ScanViewModel>()
            )
        }
        composable(
            route = Routes.CONNECTING + "/{address}",
            arguments = listOf(navArgument("address") { type = NavType.StringType })
        ) { backStackEntry ->
            ConnectingScreen(
                navController = navController,
                address = backStackEntry.arguments?.getString("address") ?: "",
                viewModel = hiltViewModel<ConnectingViewModel>()
            )
        }
        composable(Routes.DASHBOARD) {
            // TODO DashboardScreen(navController) - implement the dashboard screen
            // For now, just a placeholder
            DashboardScreen(
                navController = navController,
                viewModel = hiltViewModel<DashboardViewModel>() // Replace with actual DashboardViewModel
            )
        }
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

    fun connectingWithAddress(address: String): String {
        return "$CONNECTING/$address"
    }
}
/**
 * Placeholder theme – adopt your Material 3 color‑scheme later.
 */
@Composable
fun VisualizerTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = darkColorScheme(
            primary = Color(0xFF0066FF),
            onPrimary = Color.Black
        ),
        content = content
    )
}
