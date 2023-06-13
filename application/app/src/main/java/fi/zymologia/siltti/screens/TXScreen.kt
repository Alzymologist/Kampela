package fi.zymologia.siltti.screens

import androidx.compose.foundation.layout.Column
import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import fi.zymologia.siltti.Mode
import fi.zymologia.siltti.uniffi.Action

@Composable
fun TXScreen(
    transmitCallback: (Action?) -> Unit,
    setAppState: (Mode) -> Unit,
    packagesSent: State<Int?>,
    counterReset: () -> Unit,
) {
    KeepScreenOn()
    Column {
        /* TODO: count transmitted frames maybe?
        if (packagesSent.value == null) {
            Text("Please bring NFC receiver closer")
        } else {
            Text("Transmitted " + packagesSent.value + " packages")
        }
        */
        Text("Transmitting...")
        Text("Sent: " + packagesSent.value)
        Button(
            onClick = {
                transmitCallback(null)
                setAppState(Mode.Scan)
            },
        ) {
            Text("Stop transmission")
        }
        Button(
            onClick = {
                counterReset()
            },
        ) {
            Text("Reset counter")
        }
    }
}
