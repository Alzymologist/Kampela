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

use crate::screen::{epaper_draw_stuff_differently, epaper_draw_stuff_quickly, epaper_hw_init, epaper_deep_sleep};
use crate::ui::display_def::*;
use crate::visible_delay;

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
        let bounds = Rectangle::new(Point::zero(), Size::new(SCREEN_SIZE_X, 0));
        TextBox::with_textbox_style(self.line, bounds, character_style, textbox_style).draw(target)?;
        Ok(())
    }
}


type PixelData = BitArr!(for 176*264, in u8, Msb0);

pub struct FrameBuffer(PixelData);

impl FrameBuffer {
    pub fn new_white() -> Self {
        Self(bitarr!(u8, Msb0; 1; SCREEN_SIZE_X as usize*SCREEN_SIZE_Y as usize))
    }
    pub fn apply(&self, peripherals: &mut Peripherals) {
        epaper_hw_init(peripherals);
        epaper_draw_stuff_differently(peripherals, self.0.into_inner());
        visible_delay(10);
        epaper_deep_sleep(peripherals);
        peripherals
                    .GPIO_S
                    .if_
                    .write(|w_reg| w_reg.extif0().clear_bit());
    }
    pub fn apply_fast(&self, peripherals: &mut Peripherals) {
        epaper_hw_init(peripherals);
        epaper_draw_stuff_quickly(peripherals, self.0.into_inner());
        visible_delay(10);
        epaper_deep_sleep(peripherals);
        peripherals
                    .GPIO_S
                    .if_
                    .write(|w_reg| w_reg.extif0().clear_bit());
    }
}

impl Dimensions for FrameBuffer {
    fn bounding_box(&self) -> Rectangle {
        Rectangle {
            top_left: SCREEN_ZERO,
            size: SCREEN_SIZE,
        }
    }
}

// this was an experiment to find Y offset value in memory
//const SHIFT_COEFFICIENT: usize = (SCREEN_SIZE_Y * 7) as usize;
const SCREEN_SIZE_VALUE: usize = (SCREEN_SIZE_X*SCREEN_SIZE_Y) as usize;

impl DrawTarget for FrameBuffer {
    type Color = BinaryColor;
    type Error = DisplayError;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            if (pixel.0.x<0)|(pixel.0.x>=SCREEN_SIZE_X as i32) {return Err(DisplayError::XBounds)}
            if (pixel.0.y<0)|(pixel.0.y>=SCREEN_SIZE_Y as i32) {return Err(DisplayError::YBounds)}
            //transposing pizels correctly here
            let n = (pixel.0.y + pixel.0.x*SCREEN_SIZE_Y as i32) /*(pixel.0.y*176 + (175 - pixel.0.x))*/ as usize;
            //let n = if n<SHIFT_COEFFICIENT { n + SCREEN_SIZE_VALUE - SHIFT_COEFFICIENT } else { n - SHIFT_COEFFICIENT };
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
