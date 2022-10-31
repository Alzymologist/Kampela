package fi.zymologia.siltti.screens

import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import fi.zymologia.siltti.Mode
import fi.zymologia.siltti.uniffi.Collection

@Composable
fun TXScreen(
    collection: Collection,
    transmitCallback: (List<ByteArray>) -> Unit,
    setAppState: (Mode) -> Unit
) {
    Button(
        onClick = {
            collection.clean()
            transmitCallback(emptyList())
            setAppState(Mode.Scan)
        }
    ) {
        Text("Stop transmission")
    }
}