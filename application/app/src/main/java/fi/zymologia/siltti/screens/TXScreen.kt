package fi.zymologia.siltti.screens

import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import fi.zymologia.siltti.Mode

@Composable
fun TXScreen(
    transmitCallback: (List<ByteArray>) -> Unit,
    setAppState: (Mode) -> Unit
) {
    Button(
        onClick = {
            transmitCallback(emptyList())
            setAppState(Mode.Scan)
        }
    ) {
        Text("Stop transmission")
    }
}