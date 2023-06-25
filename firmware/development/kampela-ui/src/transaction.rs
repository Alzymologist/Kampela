use embedded_graphics::{
    mono_font::{
        ascii::{FONT_6X10},
        MonoTextStyle,
    },
    primitives::Rectangle,
    Drawable,
};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Point, Size},
    pixelcolor::BinaryColor,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};

use crate::display_def::*;

pub fn draw<D>(content: &str, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    let textbox_style = TextBoxStyleBuilder::new()
        .height_mode(HeightMode::FitToText)
        .alignment(HorizontalAlignment::Left)
        .paragraph_spacing(5)
        .build();
    let bounds = Rectangle::new(Point::zero(), Size::new(SCREEN_SIZE_X, SCREEN_SIZE_Y));
    TextBox::with_textbox_style(content, bounds, character_style, textbox_style).draw(display)?;
    Ok(())
}

