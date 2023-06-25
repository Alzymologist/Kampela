use alloc::string::String;
use bitvec::prelude::{BitArr, Msb0, bitarr};
use cortex_m::asm::delay;
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
};
use embedded_text::{
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};
use kampela_display_common::display_def::*;
use qrcodegen_noheap::{QrCode, QrCodeEcc, Version};

use crate::devices::display::{FastDraw, FullDraw, Request, epaper_draw_stuff_differently, epaper_hw_init_cs, epaper_deep_sleep};

const SCREEN_SIZE_VALUE: usize = (SCREEN_SIZE_X*SCREEN_SIZE_Y) as usize;

use crate::{in_free, parallel::Operation}; 

#[derive(Debug)]
pub enum DisplayError {}

/// These are voltage thresholds to allow screen updates;
/// for wired debug, set both well below 5000
///
//TODO tune these values for prod; something like 12k and 8k
const FAST_REFRESH_POWER: i32 = 4000;
const FULL_REFRESH_POWER: i32 = 4000;

/// Virtual display data storage
type PixelData = BitArr!(for SCREEN_SIZE_VALUE, in u8, Msb0);


/// A virtual display that could be written to EPD simultaneously
pub struct FrameBuffer {
    data: PixelData,
    display_state: DisplayState,
    timer: usize,
}

impl FrameBuffer {
    /// Create new virtual display and fill it with ON pixels
    pub fn new_white() -> Self {
        Self {
            data: bitarr!(u8, Msb0; 1; SCREEN_SIZE_X as usize*SCREEN_SIZE_Y as usize),
            display_state: DisplayState::Idle,
            timer: 0,
        }
    }

    fn count(&mut self) -> bool {
        if self.timer == 0 {
            false
        } else {
            self.timer -= 1;
            true
        }
    }

    /// Send display data to real EPD; invokes full screen refresh
    ///
    /// this is for cs environment; do not use otherwise
    fn apply(&self, peripherals: &mut Peripherals) {
        epaper_draw_stuff_differently(peripherals, self.data.into_inner());
    }

    /// Start full display update sequence
    pub fn request_full(&mut self) {
        self.display_state = DisplayState::FullRequested(Request::<FullDraw>::new());
    }

    /// Start partial fast display update sequence
    pub fn request_fast(&mut self) {
        self.display_state = DisplayState::FastRequested(Request::<FastDraw>::new());
    }
}

/// Display's updating progress
///
/// This is intentionally done without typestates, as typesafety it offers is outweighted by
/// reallocations made in new item creation.
pub enum DisplayState {
    /// Initial state, where we can change framebuffer. If this was typestate, this would be Zero.
    Idle,
    /// Fast update was requested; waiting for power
    FastRequested(Request<FastDraw>),
    /// Slow update was requested; waiting for power
    FullRequested(Request<FullDraw>),
    /// Display not available due to update cycle
    UpdatingNow,
}

impl Operation for FrameBuffer {
    type Input<'a> = i32;
    type Output = bool;
    type StateEnum = DisplayState;

    fn new() -> Self {
        Self::new_white()
    }

    fn wind(&mut self, state: DisplayState, delay: usize) {
        self.display_state = state;
        self.timer = delay;
    }

    /// Move through display update progress
    fn advance(&mut self, voltage: i32) -> bool {
        if self.count() { return false };
        match self.display_state {
            DisplayState::Idle => true,
            DisplayState::FastRequested(ref mut a) => {
                if voltage > FAST_REFRESH_POWER {        
                    if a.advance(&self.data.data) {
                        self.wind_d(DisplayState::UpdatingNow)
                    }
                };
                false
            },
            DisplayState::FullRequested(ref mut a) => {
                if voltage > FULL_REFRESH_POWER {
                    if a.advance(&self.data.data) {
                        self.wind_d(DisplayState::UpdatingNow)
                    }
                };
                false
            },
            DisplayState::UpdatingNow => {
                in_free(|peripherals| epaper_deep_sleep(peripherals));
                self.display_state = DisplayState::Idle;
                false
            },
        }
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


//**** Debug stuff ****//

/// see this <https://github.com/embedded-graphics/embedded-graphics/issues/716>
fn make_text(peripherals: &mut Peripherals, text: &str) {
    let mut buffer = FrameBuffer::new_white();
    let to_print = TextToPrint{line: text};
    to_print.draw(&mut buffer).unwrap();
    buffer.apply(peripherals);
}

struct TextToPrint<'a> {
    pub line: &'a str,
}

/// For custom font, see this <https://github.com/embedded-graphics/examples/blob/main/eg-0.7/examples/text-custom-font.rs>
impl Drawable for TextToPrint<'_> {
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
            .alignment(HorizontalAlignment::Left)
            .paragraph_spacing(5)
            .build();
        let bounds = Rectangle::new(Point::zero(), Size::new(SCREEN_SIZE_X, 0));
        TextBox::with_textbox_style(self.line, bounds, character_style, textbox_style).draw(target)?;
        Ok(())
    }
}

/// Emergency debug function that spits out errors
/// TODO: replace by power drain in production!
pub fn burning_tank(peripherals: &mut Peripherals, text: String) {
    epaper_hw_init_cs(peripherals);
    make_text(peripherals, &text);
    delay(10000000);
    epaper_deep_sleep(peripherals);
}

pub fn draw_qr(peripherals: &mut Peripherals, data_to_qr: &[u8]) {

    let len = data_to_qr.len();

    let mut outbuffer = [0u8; Version::new(18).buffer_len()].to_vec();
    let mut dataandtemp = [0u8; Version::new(18).buffer_len()].to_vec();
    
    dataandtemp[..len].copy_from_slice(data_to_qr);
    
    let qr_code = QrCode::encode_binary(&mut dataandtemp, len, &mut outbuffer, QrCodeEcc::Low, Version::MIN, Version::new(18), None, true).unwrap();

    let scaling = {
        if qr_code.version() == Version::new(18) {2}
        else {SCREEN_SIZE_Y as i32/qr_code.size()}
    };

    let mut buffer = FrameBuffer::new_white();

    let size = qr_code.size() * scaling;
    for y in 0..size {
        for x in 0..size {
            let color = {
                if qr_code.get_module(x / scaling, y / scaling) {BinaryColor::On}
                else {BinaryColor::Off}
            };
            let x_point = SCREEN_SIZE_X as i32/2 - size/2 + x;
            let y_point = SCREEN_SIZE_Y as i32/2 - size/2 + y;
            let point = Point::new(x_point, y_point);
            let pixel = Pixel::<BinaryColor>(point, color);
            pixel.draw(&mut buffer).unwrap();
        }
    }
    buffer.apply(peripherals);
}
