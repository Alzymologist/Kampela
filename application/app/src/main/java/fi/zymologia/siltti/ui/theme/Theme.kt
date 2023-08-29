package fi.zymologia.siltti.ui.theme

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material.MaterialTheme
import androidx.compose.material.darkColors
import androidx.compose.material.lightColors
import androidx.compose.runtime.Composable

private val DarkColorPalette = darkColors(
    primary = BrandPrimary,
    primaryVariant = BrandPrimaryVariant,
    secondary = BrandSecondary,
    secondaryVariant = BrandSecondary,
    background = BrandBg,
    surface = BrandSecondary,
    onPrimary = BrandOnPrimary,
    onBackground = BrandOnBackground,
)

private val LightColorPalette = lightColors(
    primary = BrandPrimary,
    primaryVariant = BrandPrimaryVariant,
    secondary = BrandSecondary,
    secondaryVariant = BrandSecondary,
    background = BrandBg,
    surface = BrandSecondary,
    onPrimary = BrandOnPrimary,
    onBackground = BrandOnBackground,

    /* Other default colors to override
    background = Color.White,
    surface = Color.White,
    onPrimary = Color.White,
    onSecondary = Color.Black,
    onBackground = Color.Black,
    onSurface = Color.Black,
    */
)

@Composable
fun SilttiTheme(darkTheme: Boolean = isSystemInDarkTheme(), content: @Composable () -> Unit) {
    val colors = if (darkTheme) {
        DarkColorPalette
    } else {
        LightColorPalette
    }

    MaterialTheme(
        colors = colors,
        typography = Typography,
        shapes = Shapes,
        content = content,
    )
}
