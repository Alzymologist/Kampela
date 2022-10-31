package fi.zymologia.siltti.screens

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import fi.zymologia.siltti.Mode

@Composable
fun NewAddress(
    setAppState: (Mode) -> Unit,
    transmitCallback: (List<ByteArray>) -> Unit
) {
    var address by remember { mutableStateOf("") }
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
        TextField(
            value = address,
            onValueChange = { address = it }
        )
        Button(
            onClick = {
                transmitCallback(listOf()) // TODO
                setAppState(Mode.TX)
            }
        ) {
            Text("Send")
        }
    }
}
