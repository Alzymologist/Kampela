//! UI state unit; almost all inerfacing should be done through this "object"

use core::ops::Add;
use embedded_graphics::{
    geometry::AnchorPoint,
    mono_font::{
        ascii::{FONT_10X20, FONT_6X10},
        MonoTextStyle,
    },
    prelude::Primitive,
    primitives::{
        Circle, Line, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
    Drawable,
};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};
use rand::seq::SliceRandom;
use rand::{rngs::ThreadRng, thread_rng};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use ux::u4;

use crate::display_def::*;

use crate::pin::Pincode;

use crate::seed_entry::SeedEntryState;

/// State of UI
pub enum UIState {
    PinEntry(Pincode),
    OnboardingRestoreOrGenerate,
    OnboardingRestore(SeedEntryState),
    OnboardingBackup(String),
    Locked,
    End,
}

impl UIState {
    pub fn new(rng: &mut ThreadRng) -> Self {
        UIState::PinEntry(Pincode::new(rng))
    }

    /// Read user touch event
    pub fn handle_event<D>(
        &mut self,
        point: Point,
        rng: &mut ThreadRng,
        fast_display: &mut D,
    ) -> Result<bool, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let mut responsive = true;
        match self {
            UIState::PinEntry(ref mut pincode) => {
                responsive = pincode.handle_event(point, rng, fast_display)?;
                match pincode.check_pin() {
                    Some(true) => {
                        println!("You win");
                        *self = UIState::OnboardingRestoreOrGenerate;
                    }
                    Some(false) => {
                        println!("kaput");
                        *self = UIState::Locked;
                    }
                    None => {}
                }
            }
            UIState::OnboardingRestoreOrGenerate => match point.x {
                0..=100 => {
                    *self = UIState::OnboardingRestore(SeedEntryState::new());
                    responsive = false;
                }
                150..=300 => {
                    *self = UIState::OnboardingBackup("".to_owned());
                    responsive = false;
                }
                _ => responsive = true,
            },
            _ => (),
            UIState::Locked => (),
            UIState::End => (),
        }
        Ok(responsive)
    }

    /// Display new screen state; should be called only when needed, is slow
    pub fn render<D>(&mut self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let clear = PrimitiveStyle::with_fill(BinaryColor::Off);
        display.bounding_box().into_styled(clear).draw(display);
        match self {
            UIState::PinEntry(ref pin) => {
                pin.draw(display)?;
            }
            UIState::Locked => {
                let linestyle = PrimitiveStyle::with_stroke(BinaryColor::On, 5);
                Line::new(
                    Point::new(0, 0),
                    Point::new(SCREEN_SIZE_X as i32, SCREEN_SIZE_Y as i32),
                )
                .into_styled(linestyle)
                .draw(display)?;
                Line::new(
                    Point::new(SCREEN_SIZE_X as i32, 0),
                    Point::new(0, SCREEN_SIZE_Y as i32),
                )
                .into_styled(linestyle)
                .draw(display)?;
            }
            _ => {}
        }
        Ok(())
    }
}
