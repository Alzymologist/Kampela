//! UI state unit; almost all inerfacing should be done through this "object"

use embedded_graphics::{
    prelude::Primitive,
    primitives::{
        Line, PrimitiveStyle},
    Drawable,
};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    pixelcolor::BinaryColor,
};

use crate::display_def::*;

use crate::platform::Platform;

use crate::seed_entry::SeedEntryState;

use crate::restore_or_generate;

pub struct EventResult {
    pub request: UpdateRequest,
    pub state: Option<Screen>,
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
pub struct UIState<P> where
    P: Platform,
{
    screen: Screen,
    platform: P,
}

pub enum Screen {
    PinEntry,
    OnboardingRestoreOrGenerate,
    OnboardingRestore(SeedEntryState),
    OnboardingBackup,
    PinRepeat,
    Locked,
    End,
}

impl <P: Platform> UIState<P> {
    pub fn new(platform: P) -> Self {
        UIState {
            screen: Screen::PinEntry,
            platform: platform,
        }
    }

    pub fn display(&mut self) -> &mut <P as Platform>::Display {
        self.platform.display()
    }

    /// Read user touch event
    pub fn handle_event<D>(
        &mut self,
        point: Point,
        h: &mut <P as Platform>::HAL,
    ) -> Result<UpdateRequest, <<P as Platform>::Display as DrawTarget>::Error>
    {
        let fast_display = self.platform.display();
        let mut out = UpdateRequest::new();
        let mut new_screen = None;
        match self.screen {
            Screen::PinEntry => {
                let res = self.platform.handle_pin_event(point, h)?;
                out = res.request;
                new_screen = res.state;
            }
            Screen::OnboardingRestoreOrGenerate => match point.x {
                0..=100 => {
                    new_screen = Some(Screen::OnboardingRestore(SeedEntryState::new()));
                    out.set_slow();
                }
                150..=300 => {
                    self.platform.generate_seed(h);
                    new_screen = Some(Screen::OnboardingBackup);
                    out.set_slow();
                }
                _ => {},
            },
            Screen::OnboardingRestore(ref mut a) => {
                let mut seed = None;
                let res = a.handle_event(point, &mut seed, fast_display)?;
                if let Some(b) = seed {
                    self.platform.set_entropy(&b);
                }
                out = res.request;
                new_screen = res.state;
            }
            Screen::OnboardingBackup => {
                new_screen = Some(Screen::PinRepeat);
                out.set_slow();
            }
            Screen::PinRepeat => {
                let res = self.platform.handle_pin_event_repeat(point, h)?;
                out = res.request;
                new_screen = res.state;
            }
            Screen::Locked => (),
            Screen::End => (),
        }
        if let Some(a) = new_screen {
           self.screen = a;
        }
        Ok(out)
    }

    /// Display new screen state; should be called only when needed, is slow
    pub fn render<D>(&mut self) -> Result<(), <<P as Platform>::Display as DrawTarget>::Error>
    {
        let display = self.platform.display();
        let clear = PrimitiveStyle::with_fill(BinaryColor::Off);
        display.bounding_box().into_styled(clear).draw(display)?;
        match self.screen {
            Screen::PinEntry => {
                self.platform.draw_pincode()?;
            }
            Screen::OnboardingRestoreOrGenerate => {
                restore_or_generate::draw(display)?;
            }
            Screen::OnboardingRestore(ref entry) => {
                entry.draw(display)?;
            }
            Screen::Locked => {
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
            Screen::OnboardingBackup => {
                self.platform.draw_backup()?;
            }
            Screen::PinRepeat => {
                self.platform.draw_pincode()?;
            }
            _ => {}
        }
        Ok(())
    }
}

