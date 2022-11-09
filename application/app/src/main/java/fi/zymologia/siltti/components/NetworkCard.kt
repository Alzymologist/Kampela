package fi.zymologia.siltti.components

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.material.Icon
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.AddCircle
import androidx.compose.material.icons.filled.CheckCircle
import androidx.compose.material.icons.filled.Delete
import androidx.compose.runtime.*
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import fi.zymologia.siltti.uniffi.SpecsKey
import fi.zymologia.siltti.uniffi.SpecsSelector

@Composable
fun NetworkCard(
    selector: MutableState<SpecsSelector>,
    key: SpecsKey
) {
    var isSelected by remember { mutableStateOf(selector.value.isSelected(key) == true) }

    Surface(
        color = if (isSelected) MaterialTheme.colors.primary else MaterialTheme.colors.secondary,
        modifier = Modifier
            .fillMaxWidth()
            .padding(10.dp)
            .clickable {
                selector.value.toggle(key)
                isSelected = selector.value.isSelected(key) == true
            }
    ) {
        Row(
            horizontalArrangement = Arrangement.Center,
            modifier = Modifier.padding(10.dp)
        ) {
            Text(
                selector.value.title(key) ?: "unknown",
                color = if (isSelected) MaterialTheme.colors.onPrimary else MaterialTheme.colors.onSecondary
            )
            if (isSelected) Icon(Icons.Default.CheckCircle, "selected") else Icon(Icons.Default.AddCircle, "not selected")
        }
    }
}
