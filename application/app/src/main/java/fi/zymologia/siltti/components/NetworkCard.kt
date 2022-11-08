package fi.zymologia.siltti.components

import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.material.MaterialTheme
import androidx.compose.material.Surface
import androidx.compose.material.Text
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
        Modifier
            .height(40.dp)
            .fillMaxWidth()
            .background(color = if (isSelected) MaterialTheme.colors.primary else MaterialTheme.colors.secondary)
            .clickable {
                selector.value.toggle(key)
                isSelected = selector.value.isSelected(key) == true
            }
    ) {
        Row(
            horizontalArrangement = Arrangement.Center
        ) {
            Text(
                selector.value.title(key) ?: "unknown",
                color = if (isSelected) MaterialTheme.colors.onPrimary else MaterialTheme.colors.onSecondary
            )
        }
    }
}
