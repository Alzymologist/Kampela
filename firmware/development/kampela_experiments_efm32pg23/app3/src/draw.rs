use bitvec::prelude::{BitArr, Msb0, bitarr};
use efm32pg23_fix::Peripherals;
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::BinaryColor,
    primitives::rectangle::Rectangle,
    Pixel,
};
use embedded_graphics::{
    Drawable,
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::Primitive,
    primitives::{Circle, PrimitiveStyle},
};
use embedded_text::{
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};

use crate::screen::epaper_draw_stuff_differently;

#[derive(Debug)]
pub enum DisplayError {
    XBounds,
    YBounds,
}

/// see this <https://github.com/embedded-graphics/embedded-graphics/issues/716>
pub fn make_text(peripherals: &mut Peripherals, text: &str) {
    let mut buffer = FrameBuffer::new_white();
    let to_print = TextToPrint{line: text};
    to_print.draw(&mut buffer).unwrap();
    buffer.apply(peripherals);
}


pub struct TextToPrint<'a> {
    pub line: &'a str,
}

/// For custom font, see this <https://github.com/embedded-graphics/examples/blob/main/eg-0.7/examples/text-custom-font.rs>
impl<'a> Drawable for TextToPrint<'a> {
    type Color = BinaryColor;
    type Output = ();
    fn draw<D>(
        &self, 
        target: &mut D
    ) -> Result<Self::Output, <D as DrawTarget>::Error>
    where
        D: DrawTarget<Color = Self::Color> 
    {
        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .height_mode(HeightMode::FitToText)
            .alignment(HorizontalAlignment::Justified)
            .paragraph_spacing(5)
            .build();
        let bounds = Rectangle::new(Point::zero(), Size::new(176, 0));
        TextBox::with_textbox_style(self.line, bounds, character_style, textbox_style).draw(target)?;
        Ok(())
    }
}


type PixelData = BitArr!(for 176*264, in u8, Msb0);

pub struct FrameBuffer(PixelData);

impl FrameBuffer {
    pub fn new_white() -> Self {
        Self(bitarr!(u8, Msb0; 1; 176*264))
    }
    pub fn apply(&self, peripherals: &mut Peripherals) {
        epaper_draw_stuff_differently(peripherals, self.0.into_inner());
    }
}

impl Dimensions for FrameBuffer {
    fn bounding_box(&self) -> Rectangle {
        Rectangle {
            top_left: Point { x: 0, y: 0 },
            size: Size {
                width: 176,
                height: 264,
            },
        }
    }
}

impl DrawTarget for FrameBuffer {
    type Color = BinaryColor;
    type Error = DisplayError;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            if (pixel.0.x<0)|(pixel.0.x>=176) {return Err(DisplayError::XBounds)}
            if (pixel.0.y<0)|(pixel.0.y>=264) {return Err(DisplayError::YBounds)}
            let n = (pixel.0.y*176 + (175 - pixel.0.x)) as usize;
            let mut pixel_update = self.0.get_mut(n).expect("checked the bounds");
            match pixel.1 {
                BinaryColor::Off => {
                    *pixel_update = true; //white
                },
                BinaryColor::On => {
                    *pixel_update = false; //black
                }
            }
        }
        Ok(())
    }
}

pub fn highlight_point(peripherals: &mut Peripherals, detected_x: u16, detected_y: u16) {
    let mut buffer = FrameBuffer::new_white();
    Circle::with_center(Point{x: 176 - detected_x as i32, y: 264 - detected_y as i32}, 20)
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut buffer).unwrap();
    buffer.apply(peripherals);
}
