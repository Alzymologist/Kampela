//! UI state unit; almost all inerfacing should be done through this "object"

use alloc::string::String;
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
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};
use rand::{Rng, seq::SliceRandom};
use ux::u4;

use patches::phrase::entropy_to_phrase;

use crate::ui::display_def::*;

use crate::ui::pin::Pincode;

use crate::ui::seed_entry::SeedEntryState;

use crate::ui::restore_or_generate;

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
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        UIState::PinEntry(Pincode::new(rng))
        //        UIState::OnboardingRestore(SeedEntryState::new())
    }

    /// Read user touch event
    pub fn handle_event<D, R: Rng + ?Sized>(
        &mut self,
        point: Point,
        rng: &mut R,
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
                        *self = UIState::OnboardingRestoreOrGenerate;
                    }
                    Some(false) => {
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
                    *self = UIState::OnboardingBackup(String::new());
                    responsive = false;
                }
                _ => responsive = true,
            },
            UIState::OnboardingRestore(ref mut a) => {
                responsive = a.handle_event(point, fast_display)?;
                if let Some(b) = a.resolve() {
                    *self = UIState::OnboardingBackup(entropy_to_phrase(&b).unwrap())
                }
            }
            UIState::OnboardingBackup(_) => {
                *self = UIState::End;
                responsive = false;
            }
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
        display.bounding_box().into_styled(clear).draw(display)?;
        match self {
            UIState::PinEntry(ref pin) => {
                pin.draw(display)?;
            }
            UIState::OnboardingRestoreOrGenerate => {
                restore_or_generate::draw(display)?;
            }
            UIState::OnboardingRestore(ref entry) => {
                entry.draw(display)?;
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
