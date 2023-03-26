//! UI state unit; almost all inerfacing should be done through this "object"

#[cfg(not(feature = "std"))]
use alloc::format;

#[cfg(feature = "std")]
use std::format;

use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    prelude::Primitive,
    primitives::{Line, PrimitiveStyle},
    Drawable,
};
use embedded_graphics_core::{draw_target::DrawTarget, geometry::Point, pixelcolor::BinaryColor};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};
use rand::{distributions::Uniform, Rng};

use crate::display_def::*;

/// State of UI
pub enum UIState {
    Crosshair(Point),
    MeasuredData(MeasuredData),
}

pub struct MeasuredData {
    pub display_point: Point,
    pub touch_point: Point,
}

impl UIState {
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        let dist_x = Uniform::new(0, SCREEN_SIZE_X as i32);
        let x: i32 = Rng::sample(rng, dist_x);
        let dist_y = Uniform::new(0, SCREEN_SIZE_Y as i32);
        let y: i32 = Rng::sample(rng, dist_y);

        UIState::Crosshair(Point { x, y })
    }

    /// Read user touch event
    pub fn process_touch<R: Rng + ?Sized>(&mut self, touch_point: Point, rng: &mut R) {
        match self {
            UIState::Crosshair(display_point) => {
                *self = UIState::MeasuredData(MeasuredData {
                    display_point: *display_point,
                    touch_point,
                });
            }
            UIState::MeasuredData(_) => {
                *self = UIState::new(rng);
            }
        }
    }

    /// Display new screen state; should be called only when needed, is slow
    pub fn render<D>(&mut self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let clear = PrimitiveStyle::with_fill(BinaryColor::Off);
        display.bounding_box().into_styled(clear).draw(display)?;
        match self {
            UIState::Crosshair(display_point) => {
                let line1_x_start = {
                    if display_point.x < HALF_LEN {
                        0
                    } else {
                        display_point.x - HALF_LEN
                    }
                };
                let line1_x_end = {
                    if display_point.x + HALF_LEN > SCREEN_SIZE_X as i32 {
                        SCREEN_SIZE_X as i32
                    } else {
                        display_point.x + HALF_LEN
                    }
                };
                Line {
                    start: Point {
                        x: line1_x_start,
                        y: display_point.y,
                    },
                    end: Point {
                        x: line1_x_end,
                        y: display_point.y,
                    },
                }
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, LINE_WIDTH))
                .draw(display)?;

                let line2_y_start = {
                    if display_point.y < HALF_LEN {
                        0
                    } else {
                        display_point.y - HALF_LEN
                    }
                };
                let line2_y_end = {
                    if display_point.y + HALF_LEN > SCREEN_SIZE_Y as i32 {
                        SCREEN_SIZE_Y as i32
                    } else {
                        display_point.y + HALF_LEN
                    }
                };
                Line {
                    start: Point {
                        x: display_point.x,
                        y: line2_y_start,
                    },
                    end: Point {
                        x: display_point.x,
                        y: line2_y_end,
                    },
                }
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, LINE_WIDTH))
                .draw(display)?;
            }
            UIState::MeasuredData(measured_data) => {
                let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

                let textbox_style = TextBoxStyleBuilder::new()
                    .alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Middle)
                    .build();

                TextBox::with_textbox_style(
                    &format!(
                        "display: ({}, {})\ntouch: ({}, {})",
                        measured_data.display_point.x,
                        measured_data.display_point.y,
                        measured_data.touch_point.x,
                        measured_data.touch_point.y
                    ),
                    display.bounding_box(),
                    character_style,
                    textbox_style,
                )
                .draw(display)?;
            }
        }
        Ok(())
    }
}
