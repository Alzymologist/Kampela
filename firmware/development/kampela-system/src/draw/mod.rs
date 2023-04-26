use alloc::string::String;

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
use kampela_display_common::display_def::*;
use cortex_m::asm::delay;

pub mod display;
use display::{display_is_busy, epaper_draw_stuff_differently, epaper_draw_stuff_quickly, epaper_hw_init, epaper_deep_sleep};
use crate::devices::power::{check_fast_display_power, check_full_display_power};

const SCREEN_SIZE_VALUE: usize = (SCREEN_SIZE_X*SCREEN_SIZE_Y) as usize;

use crate::in_free; 

#[derive(Debug)]
pub enum DisplayError {}

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

/// Virtual display data storage
type PixelData = BitArr!(for SCREEN_SIZE_VALUE, in u8, Msb0);

/// Display's updating progress
///
/// This is intentionally done without typestates, as typesafety it offers is outweighted by
/// reallocations made in new item creation.
enum DisplayState {
    /// Initial state, where we can change framebuffer. If this was typestate, this would be Zero.
    Idle,
    /// Fast update was requested; waiting for power
    FastRequested,
    /// Slow update was requested; waiting for power
    FullRequested,
    /// Display not available due to update cycle
    UpdatingNow,
}

/// A virtual display that could be written to EPD simultaneously
pub struct FrameBuffer {
    data: PixelData,
    display_state: DisplayState,
}

impl FrameBuffer {
    /// Create new virtual display and fill it with ON pixels
    pub fn new_white() -> Self {
        Self {
            data: bitarr!(u8, Msb0; 1; SCREEN_SIZE_X as usize*SCREEN_SIZE_Y as usize),
            display_state: DisplayState::Idle,
        }
    }

    /// Send display data to real EPD; invokes full screen refresh
    fn apply(&self, peripherals: &mut Peripherals) {
        epaper_hw_init(peripherals);
        epaper_draw_stuff_differently(peripherals, self.data.into_inner());
        delay(100000);
    }

    /// Send display data to real EPD in a fast partial way
    fn apply_fast(&self, peripherals: &mut Peripherals) {
        epaper_hw_init(peripherals);
        epaper_draw_stuff_quickly(peripherals, self.data.into_inner());
        delay(100000);
    }

    /// Start full display update sequence
    pub fn request_full(&mut self) {
        self.display_state = DisplayState::FullRequested;
    }

    /// Start partial fast display update sequence
    pub fn request_fast(&mut self) {
        self.display_state = DisplayState::FastRequested;
    }

    /// Move through display update progress
    pub fn advance(&mut self) -> bool {
        in_free(|peripherals| if display_is_busy(peripherals) { return });
        match self.display_state {
            DisplayState::Idle => true,
            DisplayState::FastRequested => {
                in_free(|peripherals|
                if check_fast_display_power(peripherals) {        
                    self.apply_fast(peripherals);
                    self.display_state = DisplayState::UpdatingNow;
                }
                );
                false
            },
            DisplayState::FullRequested => {
                in_free(|peripherals|
                if check_full_display_power(peripherals) {        
                    self.apply(peripherals);
                    self.display_state = DisplayState::UpdatingNow;
                }
                );
                false
            },
            DisplayState::UpdatingNow => {
                in_free(|peripherals| self.sleep(peripherals));
                false
            },
        }
    }

    /// Reset display to idle state
    fn sleep(&mut self, peripherals: &mut Peripherals) {
        epaper_deep_sleep(peripherals);
        self.display_state = DisplayState::Idle;
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

impl DrawTarget for FrameBuffer {
    type Color = BinaryColor;
    type Error = DisplayError;
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels {
            if (pixel.0.x<0)|(pixel.0.x>=SCREEN_SIZE_X as i32) {continue}
            if (pixel.0.y<0)|(pixel.0.y>=SCREEN_SIZE_Y as i32) {continue}
            //transposing pizels correctly here
            let n = (pixel.0.y + pixel.0.x*SCREEN_SIZE_Y as i32) /*(pixel.0.y*176 + (175 - pixel.0.x))*/ as usize;
            //let n = if n<SHIFT_COEFFICIENT { n + SCREEN_SIZE_VALUE - SHIFT_COEFFICIENT } else { n - SHIFT_COEFFICIENT };
            let mut pixel_update = self.data.get_mut(n).expect("checked the bounds");
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

/// Emergency debug function that spits out errors
/// TODO: replace by power drain in production!
pub fn burning_tank(peripherals: &mut Peripherals, text: String) {
    epaper_hw_init(peripherals);
    make_text(peripherals, &text);
    delay(10000000);
    epaper_deep_sleep(peripherals);
}
