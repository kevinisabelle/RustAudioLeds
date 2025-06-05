package com.kevinisabelle.visualizerui.ui.screens

import android.Manifest
import android.os.Build
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.size
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircle
import androidx.compose.material3.Button
import androidx.compose.material3.CircularProgressIndicator
import androidx.compose.material3.Icon
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.LaunchedEffect
import androidx.compose.runtime.remember
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import com.google.accompanist.permissions.ExperimentalPermissionsApi
import com.google.accompanist.permissions.MultiplePermissionsState
import com.google.accompanist.permissions.rememberMultiplePermissionsState

/**
 * First‑run “splash” that acquires runtime permissions, then hops to Scan.
 */
@OptIn(ExperimentalPermissionsApi::class)
@Composable
fun SplashPermScreen(onPermissionsGranted: () -> Unit) {
    val requiredPerms = remember {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            listOf(
                Manifest.permission.BLUETOOTH_SCAN,
                Manifest.permission.BLUETOOTH_CONNECT
            )
        } else {
            listOf(
                Manifest.permission.BLUETOOTH,
                Manifest.permission.ACCESS_FINE_LOCATION
            )
        }
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
                imageVector = Icons.Default.AddCircle,
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
                Button(onClick = {
                    permState.launchMultiplePermissionRequest()
                }) {
                    Text("Grant permissions")
                }
            } else {
                CircularProgressIndicator(color = MaterialTheme.colorScheme.onPrimary)
            }
        }
    }
}
