//! UI state unit; almost all inerfacing should be done through this "object"

#[cfg(not(feature = "std"))]
use alloc::{borrow::ToOwned, format, vec::Vec};

#[cfg(not(feature = "std"))]
use core::convert::TryInto;

#[cfg(feature = "std")]
use std::{borrow::ToOwned, convert::TryInto, format, vec::Vec};

use embedded_graphics::{
    mono_font::{
        ascii::{FONT_10X20, FONT_7X13},
        MonoTextStyle,
    },
    prelude::Primitive,
    primitives::{Line, PrimitiveStyle, Rectangle},
    Drawable,
};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    pixelcolor::BinaryColor,
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::TextBoxStyleBuilder,
    TextBox,
};
use rand::{distributions::Uniform, Rng};

use kampela_display_common::display_def::{SCREEN_SIZE_X, SCREEN_SIZE_Y};

use crate::calibration::{MeasuredAffine, SET_LEN};
use crate::display_def::*;

/// State of UI
pub enum UIState {
    Crosshair {
        point: Point,
        collected_data: Vec<MeasuredData>,
    },
    MeasuredData {
        measured_data: MeasuredData,
        collected_data: Vec<MeasuredData>,
    },
    Complete(MeasuredAffine),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeasuredData {
    pub display_point: Point,
    pub touch_point: Point,
}

fn random_point<R: Rng + ?Sized>(rng: &mut R) -> Point {
    let dist_x = Uniform::new(0, SCREEN_SIZE_X as i32);
    let x: i32 = Rng::sample(rng, dist_x);
    let dist_y = Uniform::new(0, SCREEN_SIZE_Y as i32);
    let y: i32 = Rng::sample(rng, dist_y);

    Point { x, y }
}

impl UIState {
    pub fn init<R: Rng + ?Sized>(rng: &mut R) -> Self {
        UIState::Crosshair {
            point: random_point(rng),
            collected_data: Vec::new(),
        }
    }

    pub fn next_crosshair<R: Rng + ?Sized>(rng: &mut R, collected_data: Vec<MeasuredData>) -> Self {
        UIState::Crosshair {
            point: random_point(rng),
            collected_data,
        }
    }

    /// Read user touch event
    pub fn process_touch<R: Rng + ?Sized>(
        &mut self,
        touch_point: Point,
        rng: &mut R,
    ) -> Result<bool, &'static str> {
        match self {
            UIState::Crosshair {
                point,
                collected_data,
            } => {
                let measured_data = MeasuredData {
                    display_point: *point,
                    touch_point,
                };
                *self = UIState::MeasuredData {
                    measured_data,
                    collected_data: collected_data.to_vec(),
                };
                Ok(true)
            }
            UIState::MeasuredData {
                measured_data,
                collected_data,
            } => {
                if APPROVE_BUTTON_AREA.contains(touch_point) {
                    let mut collected_data_updated = collected_data.to_vec();
                    collected_data_updated.push(measured_data.to_owned());

                    if collected_data_updated.len() == SET_LEN {
                        let measured_data_array: [MeasuredData; SET_LEN] = collected_data_updated
                            .try_into()
                            .expect("static length, always fit");
                        let measured_affine = MeasuredAffine::from_data(&measured_data_array)?;
                        *self = UIState::Complete(measured_affine);
                    } else {
                        *self = UIState::next_crosshair(rng, collected_data_updated);
                    }
                    Ok(true)
                } else if DECLINE_BUTTON_AREA.contains(touch_point) {
                    *self = UIState::next_crosshair(rng, collected_data.to_vec());
                    Ok(true)
                } else if RESTART_BUTTON_AREA.contains(touch_point) {
                    *self = UIState::init(rng);
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            UIState::Complete(_) => Ok(false),
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
            UIState::Crosshair {
                point,
                collected_data: _,
            } => {
                let line1_x_start = {
                    if point.x < HALF_LEN {
                        0
                    } else {
                        point.x - HALF_LEN
                    }
                };
                let line1_x_end = {
                    if point.x + HALF_LEN > SCREEN_SIZE_X as i32 {
                        SCREEN_SIZE_X as i32
                    } else {
                        point.x + HALF_LEN
                    }
                };
                Line {
                    start: Point {
                        x: line1_x_start,
                        y: point.y,
                    },
                    end: Point {
                        x: line1_x_end,
                        y: point.y,
                    },
                }
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, LINE_WIDTH))
                .draw(display)?;

                let line2_y_start = {
                    if point.y < HALF_LEN {
                        0
                    } else {
                        point.y - HALF_LEN
                    }
                };
                let line2_y_end = {
                    if point.y + HALF_LEN > SCREEN_SIZE_Y as i32 {
                        SCREEN_SIZE_Y as i32
                    } else {
                        point.y + HALF_LEN
                    }
                };
                Line {
                    start: Point {
                        x: point.x,
                        y: line2_y_start,
                    },
                    end: Point {
                        x: point.x,
                        y: line2_y_end,
                    },
                }
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, LINE_WIDTH))
                .draw(display)?;
            }
            UIState::MeasuredData {
                measured_data,
                collected_data,
            } => {
                let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);

                let textbox_style = TextBoxStyleBuilder::new()
                    .alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Middle)
                    .build();

                TextBox::with_textbox_style(
                    &format!(
                        "point {} of {}\n\ndisplay: ({}, {})\ntouch: ({}, {})",
                        collected_data.len() + 1,
                        SET_LEN,
                        measured_data.display_point.x,
                        measured_data.display_point.y,
                        measured_data.touch_point.x,
                        measured_data.touch_point.y
                    ),
                    DISPLAY_DATA_AREA.bounding_box(),
                    character_style,
                    textbox_style,
                )
                .draw(display)?;

                control_button::<D>("restart", &RESTART_BUTTON_AREA, display)?;

                control_button::<D>("skip", &DECLINE_BUTTON_AREA, display)?;

                if collected_data.len() + 1 == SET_LEN {
                    control_button::<D>("done", &APPROVE_BUTTON_AREA, display)?;
                } else {
                    control_button::<D>("add", &APPROVE_BUTTON_AREA, display)?;
                }
            }
            UIState::Complete(measured_affine) => {
                let character_style = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);

                let textbox_style = TextBoxStyleBuilder::new()
                    .alignment(HorizontalAlignment::Center)
                    .vertical_alignment(VerticalAlignment::Middle)
                    .build();

                TextBox::with_textbox_style(
                    &measured_affine.show(),
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

fn control_button<D>(text: &str, bounds: &Rectangle, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let character_style = MonoTextStyle::new(&FONT_7X13, BinaryColor::On);
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

    (*bounds).into_styled(thin_stroke).draw(display)?;

    let textbox_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Middle)
        .build();

    TextBox::with_textbox_style(text, *bounds, character_style, textbox_style).draw(display)?;
    Ok(())
}
