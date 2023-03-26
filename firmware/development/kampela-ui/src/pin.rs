//! Pin code entry screen

#[cfg(not(feature="std"))]
use alloc::format;

#[cfg(feature="std")]
use std::format;

use bitvec::prelude::{bitarr, BitArr, Msb0};
use embedded_graphics::{
    geometry::AnchorPoint,
    mono_font::{
        ascii::{FONT_10X20, FONT_6X10},
        MonoTextStyle,
    },
    prelude::*,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
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

use crate::display_def::*;
use crate::uistate::{EventResult, UIState, UpdateRequest};

/// Displayed size of pin button
const PIN_BUTTON_SIZE: Size = Size::new(40, 40);
const PIN_BUTTON_ACTIVE_SIZE: Size = Size::new(30, 30);

/// Number of pin buttons
const PIN_BUTTON_COUNT: usize = 16;

/// Positions of button centers on screen
const PIN_BUTTON_POSITIONS: [Point; PIN_BUTTON_COUNT] = {
    let mut positions: [Point; PIN_BUTTON_COUNT] = [Point::new(0, 0); PIN_BUTTON_COUNT];
    let x_offset = 110;
    let y_offset = 22;
    let x_gap = GAP;
    let y_gap = GAP;
    let x_spacing = x_gap + PIN_BUTTON_SIZE.width;
    let y_spacing = y_gap + PIN_BUTTON_SIZE.height;

    let mut x = 0;
    let mut y = 0;
    while x < 4 && y < 4 {
        positions[x + y * 4] = Point::new(
            ((x as u32) * x_spacing + x_offset) as i32,
            ((y as u32) * y_spacing + y_offset) as i32,
        ); //offset.add(Size::new(x, y).component_mul(gap.saturating_add(PIN_BUTTON_SIZE)));
        if x == 3 {
            x = 0;
            y += 1;
        } else {
            x += 1;
        }
    }
    positions
};

/// Number of pin digits
const PIN_LEN: usize = 6;

const PIN_COUNTER_DIAMETER: u32 = 16;

/// Positions of pin code counter dots
const PIN_COUNT_POSITIONS: [Point; PIN_LEN] = {
    let mut out = [Point::new(0, 0); PIN_LEN];
    let x_offset = 45;
    let y_offset = 50;
    let y_spacing = GAP + PIN_COUNTER_DIAMETER;
    let mut i = 0;
    while i < PIN_LEN {
        out[i] = Point::new(x_offset, ((i as u32) * y_spacing + y_offset) as i32);
        i += 1;
    }
    out
};

lazy_static! {
    /// Visible areas of buttons
    static ref PIN_BUTTON_AREA: [Rectangle; PIN_BUTTON_COUNT] = {
        let mut output: [Rectangle; PIN_BUTTON_COUNT] = [Rectangle::zero(); PIN_BUTTON_COUNT];
        for i in 0..PIN_BUTTON_COUNT {
            output[i] = Rectangle::with_center(PIN_BUTTON_POSITIONS[i], PIN_BUTTON_SIZE);
        };
        output
    };

    /// Touchable areas of buttons
    static ref PIN_BUTTON_AREA_ACTIVE: [Rectangle; PIN_BUTTON_COUNT] = {
        let mut output: [Rectangle; PIN_BUTTON_COUNT] = [Rectangle::zero(); PIN_BUTTON_COUNT];

        for i in 0..PIN_BUTTON_COUNT {
            output[i] = PIN_BUTTON_AREA[i].resized(PIN_BUTTON_ACTIVE_SIZE, AnchorPoint::Center);
        };
        output
    };

    /// Pin counter circles
    static ref PIN_COUNTER_AREA: [Circle; PIN_LEN] = {
        let mut out = [Circle::new(Point::zero(), 0); PIN_LEN];
        for i in 0..PIN_LEN {
            out[i] = Circle::with_center(PIN_COUNT_POSITIONS[i], PIN_COUNTER_DIAMETER);
        };
        out
    };
}

/// Draw a pin code button
fn pin_button<D>(number: &u4, bounds: &Rectangle, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

    bounds.clone().into_styled(thin_stroke).draw(display)?;

    let textbox_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Middle)
        .build();

    TextBox::with_textbox_style(
        &format!("{:x}", number),
        *bounds,
        character_style,
        textbox_style,
    )
    .draw(display)?;
    Ok(())
}

/// Draw a pushed pin code button
fn pin_button_pushed<D>(number: &u4, bounds: &Rectangle, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::Off);
    let filled = PrimitiveStyle::with_fill(BinaryColor::On);

    bounds.clone().into_styled(filled).draw(display)?;

    let textbox_style = TextBoxStyleBuilder::new()
        .alignment(HorizontalAlignment::Center)
        .vertical_alignment(VerticalAlignment::Middle)
        .build();

    TextBox::with_textbox_style(
        &format!("{:x}", number),
        *bounds,
        character_style,
        textbox_style,
    )
    .draw(display)?;
    Ok(())
}

/// Shuffle keys
fn get_pinkeys<R: Rng + ?Sized>(rng: &mut R) -> [u4; 16] {
    let mut pinset: [u4; 16] = core::array::from_fn(|i| {
        (i).try_into()
            .expect("static initialization of numbers 0..15")
    });
    pinset.shuffle(rng);
    pinset
}

fn pin_counter<D>(on: bool, bounds: &Circle, display: &mut D) -> Result<(), D::Error>
where
    D: DrawTarget<Color = BinaryColor>,
{
    let medium_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 2);
    if on {
        let filled = PrimitiveStyle::with_fill(BinaryColor::On);
        bounds
            .clone()
            .offset(-3)
            .into_styled(filled)
            .draw(display)?;
        bounds.clone().into_styled(medium_stroke).draw(display)?;
    } else {
        bounds.clone().into_styled(medium_stroke).draw(display)?;
    };
    Ok(())
}

const PIN_CODE_MOCK: [u4; PIN_LEN] = [u4::new(0); PIN_LEN];

/// UI state for pin code entry stage
pub struct Pincode {
    code: [u4; PIN_LEN],
    position: usize,
    permutation: [u4; PIN_BUTTON_COUNT],
}

impl Pincode {
    pub fn new<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Pincode {
            code: [u4::new(0); PIN_LEN],
            position: 0,
            permutation: get_pinkeys(rng),
        }
    }

    /// Change pin keys positions; remember to run before new key press
    fn shuffle<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        self.permutation = get_pinkeys(rng);
    }

    /// User pushed a button
    pub fn input<R: Rng + ?Sized>(&mut self, rng: &mut R, key: u4) {
        self.code[self.position] = key;
        self.position += 1;
        self.shuffle(rng);
    }

    fn handle_button<D, R: Rng + ?Sized>(
        &mut self,
        point: Point,
        rng: &mut R,
        fast_display: &mut D,
    ) -> Result<UpdateRequest, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let mut out = UpdateRequest::new();
        for (index, area) in PIN_BUTTON_AREA_ACTIVE.iter().enumerate() {
            if area.contains(point) {
                let key = self.permutation[index].clone();
                pin_button_pushed(&key, &PIN_BUTTON_AREA[index], fast_display)?;
                self.input(rng, key);
                out.set_both();
                break;
            }
        }
        Ok(out)
    }


    /// Input event (user touched screen in pin entry mode)
    pub fn handle_event<D, R: Rng + ?Sized>(
        &mut self,
        point: Point,
        rng: &mut R,
        fast_display: &mut D,
    ) -> Result<EventResult, D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let request = self.handle_button(point, rng, fast_display)?;
        let state = self.check_pin();
        Ok(EventResult {request, state})
    }

    /// Check pin code; decision making for whether to leave this screen and how
    fn check_pin(&self) -> Option<UIState> {
        if self.position == PIN_LEN {
            if self.code == PIN_CODE_MOCK {
                Some(UIState::OnboardingRestoreOrGenerate)
            } else {
                Some(UIState::Locked)
            }
        } else {
            None
        }
    }

    pub fn draw_counter<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();
        let bounds = Rectangle::new(Point::new(4, 4), Size::new(90, 50));

        TextBox::with_textbox_style("Enter pin", bounds, character_style, textbox_style)
            .draw(display)?;

        for i in 0..PIN_LEN {
            pin_counter(i < self.position, &PIN_COUNTER_AREA[i], display)?;
        }
        Ok(())
    }

    pub fn draw_pinpad<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        for i in 0..PIN_BUTTON_COUNT {
            pin_button(&self.permutation[i], &PIN_BUTTON_AREA[i], display)?;
        }
        Ok(())
    }

    /// Draw whole pin code pad
    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = BinaryColor>,
    {
        self.draw_pinpad(display)?;
        self.draw_counter(display)?;
        Ok(())
    }
}
