//! UI state unit; almost all inerfacing should be done through this "object"

#[cfg(not(feature="std"))]
use alloc::string::String;

#[cfg(feature="std")]
use std::string::String;

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

use crate::display_def::*;

use crate::pin::Pincode;

use crate::seed_entry::SeedEntryState;

use crate::restore_or_generate;

pub struct EventResult {
    pub request: UpdateRequest,
    pub state: Option<UIState>,
}

impl EventResult {
    pub fn new() -> Self {
        EventResult {
            request: UpdateRequest::new(),
            state: None,
        }
    }
}

pub struct UpdateRequest {
    fast: bool,
    slow: bool,
}

impl UpdateRequest {
    pub fn new() -> Self {
        UpdateRequest {
            fast: false,
            slow: false,
        }
    }

    pub fn set_slow(&mut self) {
        self.slow = true;
    }

    pub fn set_fast(&mut self) {
        self.fast = true;
    }

    pub fn set_both(&mut self) {
        self.set_slow();
        self.set_fast();
    }

    pub fn propagate(&mut self, mut new: UpdateRequest) {
        if new.read_fast() { self.set_fast() };
        if new.read_slow() { self.set_slow() };
    }

    pub fn read_slow(&mut self) -> bool {
        if self.slow {
            self.slow = false;
            true
        } else { false }
    }

    pub fn read_fast(&mut self) -> bool {
        if self.fast {
            self.fast = false;
            true
        } else { false }
    }
}

impl Default for UpdateRequest {
    fn default() -> Self {
        Self::new()
    }
}

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
    ) -> Result<UpdateRequest, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let mut out = UpdateRequest::new();
        match self {
            UIState::PinEntry(ref mut pincode) => {
                let res = pincode.handle_event(point, rng, fast_display)?;
                out = res.request;
                if let Some(a) = res.state {
                    *self = a;
                }
            }
            UIState::OnboardingRestoreOrGenerate => match point.x {
                0..=100 => {
                    *self = UIState::OnboardingRestore(SeedEntryState::new());
                    out.set_slow();
                }
                150..=300 => {
                    *self = UIState::OnboardingBackup(String::new());
                    out.set_slow();
                }
                _ => {},
            },
            UIState::OnboardingRestore(ref mut a) => {
                let res = a.handle_event(point, fast_display)?;
                out = res.request;
                if let Some(b) = res.state {
                    *self = b;
                }
            }
            UIState::OnboardingBackup(_) => {
                *self = UIState::End;
                out.set_slow();
            }
            UIState::Locked => (),
            UIState::End => (),
        }
        Ok(out)
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

