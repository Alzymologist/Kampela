package fi.zymologia.siltti.screens

import android.util.Log
import androidx.compose.foundation.layout.Column
import androidx.compose.material.Button
import androidx.compose.material.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.State
import fi.zymologia.siltti.Mode

@Composable
fun TXScreen(
    transmitCallback: (List<ByteArray>) -> Unit,
    setAppState: (Mode) -> Unit,
    packagesSent: State<Int?>
) {
    Column {
        /* TODO: count transmitted frames maybe?
        if (packagesSent.value == null) {
            Text("Please bring NFC receiver closer")
        } else {
            Text("Transmitted " + packagesSent.value + " packages")
        }
        */
        Text("Transmitting...")
        Button(
            onClick = {
                transmitCallback(emptyList())
                setAppState(Mode.Scan)
            }
        ) {
            Text("Stop transmission")
        }
    }
}