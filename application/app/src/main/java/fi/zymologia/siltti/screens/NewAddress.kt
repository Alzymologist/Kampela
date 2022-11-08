package fi.zymologia.siltti.screens

import androidx.compose.foundation.BorderStroke
import androidx.compose.foundation.border
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.*
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.unit.dp
import fi.zymologia.siltti.Mode
import fi.zymologia.siltti.components.NetworkCard
import fi.zymologia.siltti.uniffi.SpecsSelector

@Composable
fun NewAddress(
    setAppState: (Mode) -> Unit,
    transmitCallback: (List<ByteArray>) -> Unit,
    dbName: String
) {
    var address by remember { mutableStateOf("") }
    var selector = remember { mutableStateOf(SpecsSelector(dbName)) }

    Column(
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
        LazyColumn {
            this.items(
                items = selector.value.getAllKeys(),
                key = { it }
            ) { key ->
                NetworkCard(selector, key)
            }
        }
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