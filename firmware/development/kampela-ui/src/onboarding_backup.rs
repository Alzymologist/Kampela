use embedded_graphics::{
    geometry::AnchorPoint,
mono_font::{
        ascii::{FONT_10X20, FONT_4X6, FONT_6X10},
        MonoTextStyle,
    },
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    Drawable,
};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};

use patches::phrase::{phrase_to_entropy, wordlist_english, Bits11, WordList, WordListElement};

use crate::display_def::*;

/// Show seed to user
pub fn draw<D>(seed: &str, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let textbox_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Middle)
        .build();

    let restore = Rectangle::new(
        Point::new(0, 50),
        Size::new(SCREEN_SIZE_X / 2, SCREEN_SIZE_Y - 50),
    );
    let generate = Rectangle::new(
        Point::new(SCREEN_SIZE_X as i32 / 2, 50),
        Size::new(SCREEN_SIZE_X / 2, SCREEN_SIZE_Y - 50),
    );
    let header = Rectangle::new(Point::new(0, 0), Size::new(SCREEN_SIZE_X, 50));
    TextBox::with_textbox_style("restore", restore, character_style, textbox_style)
        .draw(display)?;
    TextBox::with_textbox_style("generate", generate, character_style, textbox_style)
        .draw(display)?;
    TextBox::with_textbox_style(
        "Your seed is:",
        header,
        character_style,
        textbox_style,
    )
    .draw(display)?;
    Ok(())
}
