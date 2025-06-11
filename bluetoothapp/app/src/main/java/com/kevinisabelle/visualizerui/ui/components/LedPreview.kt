package com.kevinisabelle.visualizerui.ui.components

import androidx.compose.foundation.Canvas
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.tooling.preview.Preview
import androidx.compose.ui.unit.dp

/** Draws the LED matrix as coloured squares. */
@Composable
fun LedPreview(
    colors: List<Color>,
    columns: Int,
    rows: Int,
    modifier: Modifier = Modifier.Companion
) {
    Canvas(modifier) {
        if (colors.isEmpty()) return@Canvas
        val cellW = size.width / columns
        val cellH = size.height / rows
        for (x in 0 until columns) {
            for (y in 0 until rows) {
                val idx = x * rows + y
                if (idx < colors.size) {
                    val isEvenColumns = x % 2 == 0
                    // If is odd column, reverse the y index
                    val yReversed = if (isEvenColumns) rows - 1 - y else y
                    drawRect(
                        color = colors[idx],
                        topLeft = Offset(x * cellW, yReversed * cellH),
                        size = Size(cellW, cellH)
                    )
                }
            }
        }
    }
}

@Preview
@Composable
fun LedPreviewPreview() {
    LedPreview(
        colors = List(22 * 12) { Color(it / 255f, it / 255f, it / 255f) },
        columns = 22,
        rows = 12,
        modifier = Modifier
            .fillMaxWidth()
            .height(200.dp)
    )
}
