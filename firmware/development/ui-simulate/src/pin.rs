use bitvec::prelude::{BitArr, Msb0, bitarr};
use embedded_graphics_core::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point, Size},
    pixelcolor::BinaryColor,
    Pixel,
};
use embedded_graphics::{
    Drawable,
    geometry::AnchorPoint,
    mono_font::{ascii::{FONT_6X10, FONT_10X20}, MonoTextStyle},
    prelude::Primitive,
    primitives::{
        Circle, PrimitiveStyle, PrimitiveStyleBuilder, Rectangle, StrokeAlignment, Triangle,
    },
};
use embedded_text::{
    alignment::{HorizontalAlignment, VerticalAlignment},
    style::{HeightMode, TextBoxStyleBuilder},
    TextBox,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use ux::u4;
use rand::seq::SliceRandom;
use rand::{rngs::ThreadRng, thread_rng};


/// Displayed size of pin button
const PIN_BUTTON_SIZE: Size = Size::new(40, 40);
const PIN_BUTTON_ACTIVE_SIZE: Size = Size::new(30, 30);

/// Number of pin buttons
const PIN_BUTTON_COUNT: usize = 16;

/// Positions of button centers on screen
const PIN_BUTTON_POSITIONS: [Point; PIN_BUTTON_COUNT] = {
    let mut positions: [Point; PIN_BUTTON_COUNT] = [Point::new(0, 0); PIN_BUTTON_COUNT];
    let mut counter = 0;
    let x_offset = 110;
    let y_offset = 22;
    let x_gap = 4;
    let y_gap = 4;
    let x_spacing = x_gap + PIN_BUTTON_SIZE.width;
    let y_spacing = y_gap + PIN_BUTTON_SIZE.height;
    let offset = Point::new(90, 2);
    let gap = Size::new(4, 4);

    let mut x = 0;
    let mut y = 0;
    while x<4 && y<4 {
        positions[x+y*4] = Point::new(((x as u32)*x_spacing + x_offset) as i32, ((y as u32)*y_spacing + y_offset) as i32);//offset.add(Size::new(x, y).component_mul(gap.saturating_add(PIN_BUTTON_SIZE)));
        if x == 3 {
            x = 0;
            y += 1;
        } else {
            x += 1;
        }
    }
    positions
};

/// Visible areas of buttons
lazy_static! {
    static ref PIN_BUTTON_AREA: [Rectangle; PIN_BUTTON_COUNT] = {
        let mut output: [Rectangle; PIN_BUTTON_COUNT] = [Rectangle::zero(); PIN_BUTTON_COUNT];
        for i in 0..PIN_BUTTON_COUNT {
            output[i] = Rectangle::with_center(PIN_BUTTON_POSITIONS[i], PIN_BUTTON_SIZE);
        };
        output
    };

    static ref PIN_BUTTON_AREA_ACTIVE: [Rectangle; PIN_BUTTON_COUNT] = {
        let mut output: [Rectangle; PIN_BUTTON_COUNT] = [Rectangle::zero(); PIN_BUTTON_COUNT];

        for i in 0..PIN_BUTTON_COUNT {
            output[i] = PIN_BUTTON_AREA[i].resized(PIN_BUTTON_ACTIVE_SIZE, AnchorPoint::Center);
        };
        output
    };
}

/// Draw a pin code button
fn pin_button<D>(number: &u4, bounds: &Rectangle, display: &mut D) -> Result<(), D::Error>
    where D: DrawTarget<Color = BinaryColor> 
{
    let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

    bounds.clone()
        .into_styled(thin_stroke)
        .draw(display)?;

    let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

    TextBox::with_textbox_style(&format!("{:x}", number), bounds.to_owned(), character_style, textbox_style).draw(display)?;
    Ok(())
}

/// Draw a pushed pin code button
fn pin_button_pushed<D>(number: &u4, bounds: &Rectangle, display: &mut D) -> Result<(), D::Error>
    where D: DrawTarget<Color = BinaryColor> 
{
    let character_style = MonoTextStyle::new(&FONT_10X20, BinaryColor::Off);
    let filled = PrimitiveStyle::with_fill(BinaryColor::On);

    bounds.clone()
        .into_styled(filled)
        .draw(display)?;

    let textbox_style = TextBoxStyleBuilder::new()
            .alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Middle)
            .build();

    TextBox::with_textbox_style(&format!("{:x}", number), bounds.to_owned(), character_style, textbox_style).draw(display)?;
    Ok(())
}

/// Shuffle keys
fn get_pinkeys(rng: &mut ThreadRng) -> [u4; 16] {
    let mut pinset: [u4; 16] = core::array::from_fn(|i| (i).try_into().expect("static initialization of numbers 0..15"));
    pinset.shuffle(rng);
    pinset
}


const PIN_LEN: usize = 4;

const PIN_CODE_MOCK: [u4; PIN_LEN] = [u4::new(0); PIN_LEN];

/// UI state for pin code entry stage
pub struct Pincode {
    code: [u4; PIN_LEN],
    position: usize,
    permutation: [u4; PIN_BUTTON_COUNT],
}

impl Pincode {
    pub fn new(rng: &mut ThreadRng) -> Self {
        Pincode {
            code: [u4::new(0); PIN_LEN],
            position: 0,
            permutation: get_pinkeys(rng),
        }
    }

    /// Change pin keys positions; remember to run before new key press
    fn shuffle(&mut self, rng: &mut ThreadRng) {
        self.permutation = get_pinkeys(rng);
    }

    /// User pushed a button
    pub fn input(&mut self, rng: &mut ThreadRng, key: u4) {
        self.code[self.position] = key;
        self.position += 1;
        self.shuffle(rng);
    }
    
    /// Input event (user touched screen in pin entry mode)
    pub fn handle_event<D>(&mut self, point: Point, rng: &mut ThreadRng, fast_display: &mut D) -> Result<bool, D::Error>
        where D: DrawTarget<Color = BinaryColor> 
    {
        let mut responsive = true;
        for (index, area) in PIN_BUTTON_AREA_ACTIVE.iter().enumerate() {
            if area.contains(point) {
                let key = self.permutation[index];
                println!("User pressed button {}", key);
                pin_button_pushed(&key, &PIN_BUTTON_AREA[index], fast_display);
                self.input(rng, key);
                responsive = false;
                break;
            }
        }
        Ok(responsive)
    }

    /// Check pin code; decision making for whether to leave this screen and how
    pub fn check_pin(&self) -> Option<bool> {
        if self.position == 4 {
            if self.code == PIN_CODE_MOCK {
                Some(true)
            } else {
                Some(false)
            }
        } else {
            None
        }
    }

    /// Draw whole pin code pad
    pub fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
        where D: DrawTarget<Color = BinaryColor> 
    {
        for i in 0..PIN_BUTTON_COUNT {
        pin_button(&self.permutation[i], &PIN_BUTTON_AREA[i], display)?;
    }
    Ok(())
}

}

