package com.kevinisabelle.visualizerui.ui.components

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.width
import androidx.compose.runtime.Composable
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp
import com.godaddy.android.colorpicker.ClassicColorPicker
import com.godaddy.android.colorpicker.HsvColor
import com.kevinisabelle.visualizerui.data.Rgb888

@Composable
fun ColorPicker(
    modifier: Modifier = Modifier,
    color: Rgb888,
    onColorSelected: (Rgb888) -> Unit = { }
) {
    val selectedColor = remember { mutableStateOf(color) }

    Column(modifier = modifier)
    {
        // Draw a swatch to show the selected color
        Row(
            modifier = Modifier
                .width(100.dp)
                .height(20.dp)
                .background(selectedColor.value.toStdColor())
        ) {
0            // This row will show the selected color as a swatch
        }
        ClassicColorPicker(
            color = HsvColor.from(color.toStdColor()),
            showAlphaBar = false,
            onColorChanged = { color: HsvColor ->
                // Update the selected color when the user picks a new one
                selectedColor.value = Rgb888.fromStdColor(color.toColor())
                // Convert the selected color back to Rgb888 and notify the listener
                onColorSelected(selectedColor.value)
            },
            modifier = Modifier.width(100.dp).weight(1f, true)
        )

    }
}

@Preview
@Composable
fun ColorPickerComponentPreview() {
    ColorPicker(color = Rgb888.fromStdColor(Color.Blue), onColorSelected = { newColor ->

        println("Color 1 set to: $newColor")
    }, modifier = Modifier.width(100.dp).height(200.dp))
}