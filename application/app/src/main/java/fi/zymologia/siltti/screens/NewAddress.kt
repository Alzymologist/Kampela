package fi.zymologia.siltti.screens

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.Button
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import fi.zymologia.siltti.Mode

@Composable
fun NewAddress(setAppState: (Mode) -> Unit) {
    Surface(
        Modifier
            .padding(bottom = 24.dp)
            .border(
                BorderStroke(1.dp, MaterialTheme.colors.primary),
                RoundedCornerShape(8.dp)
            )
            .clip(RoundedCornerShape(8.dp))
    ) {
        Text("Type derivation")
        Button(
            onClick = { setAppState(Mode.TX) }
        ) {
            Text("crach the app")
        }
    }
}