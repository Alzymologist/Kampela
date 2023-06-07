//! Screen for seed phrase display

use patches::entropy_to_phrase;
use embedded_graphics::{
    mono_font::{
        ascii::{FONT_8X13_BOLD},
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
    style::TextBoxStyleBuilder,
    TextBox,
};

use crate::display_def::*;

pub fn draw_backup_screen<D: DrawTarget<Color = BinaryColor>>(entropy: &[u8], display: &mut D) -> Result<(), D::Error> {
    let character_style = MonoTextStyle::new(&FONT_8X13_BOLD, BinaryColor::On);
    let textbox_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Middle)
        .build();
    let header = Rectangle::new(Point::new(0, 4), Size::new(SCREEN_SIZE_X, 24));
    let body = Rectangle::new(Point::new(0, 28), Size::new(SCREEN_SIZE_X, 100));
    let bottom = Rectangle::new(Point::new(0, 132), Size::new(SCREEN_SIZE_X, 50));

    match entropy_to_phrase(entropy) {
        Ok(ref seed) => {
            TextBox::with_textbox_style("Please write down seed phrase", header, character_style, textbox_style).draw(display)?;
            TextBox::with_textbox_style(seed, body, character_style, textbox_style).draw(display)?;
            TextBox::with_textbox_style("touch the screen when done", bottom, character_style, textbox_style).draw(display)?;
        },
        Err(_e) => {
            TextBox::with_textbox_style("System error! Seed storage corrupted; if this persists, please destroy the device", body, character_style, textbox_style).draw(display)?;
        },
    };
    
    Ok(())
}
