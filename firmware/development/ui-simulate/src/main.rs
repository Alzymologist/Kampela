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
use core::ops::Add;
use std::{time::{Duration, Instant}, thread::sleep};

#[macro_use]
extern crate lazy_static;

/// Amount of time required for full screen update
const SLOW_UPDATE_TIME: Duration = Duration::new(1, 0);

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


fn get_pinkeys(rng: &mut ThreadRng) -> [u4; 16] {
    let mut pinset: [u4; 16] = core::array::from_fn(|i| (i).try_into().expect("static initialization of numbers 0..15"));
    pinset.shuffle(rng);
    pinset
}

fn pinpad<D>(pinset: &[u4; 16], display: &mut D) -> Result<(), D::Error>
    where D: DrawTarget<Color = BinaryColor> 
{
        for i in 0..PIN_BUTTON_COUNT {
        pin_button(&pinset[i], &PIN_BUTTON_AREA[i], display)?;
    }
    Ok(())
}

const PIN_LEN: usize = 4;

const PIN_CODE_MOCK: [u4; PIN_LEN] = [u4::new(0); PIN_LEN];

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

    fn shuffle(&mut self, rng: &mut ThreadRng) {
        self.permutation = get_pinkeys(rng);
    }

    pub fn input(&mut self, rng: &mut ThreadRng, key: u4) {
        self.code[self.position] = key;
        self.position += 1;
        self.shuffle(rng);
    }
}

enum UIState {
    PinEntry(Pincode),
    Locked,
    End,
}

impl UIState {
    pub fn new(rng: &mut ThreadRng) -> Self {
        UIState::PinEntry(Pincode::new(rng))
    }

    pub fn handle_event<D>(&mut self, point: Point, rng: &mut ThreadRng, fast_display: &mut D) -> Result<bool, D::Error>
        where D: DrawTarget<Color = BinaryColor> 
    {
        let mut responsive = true;
        match self {
            UIState::PinEntry(ref mut pincode) => {
                for (index, area) in PIN_BUTTON_AREA_ACTIVE.iter().enumerate() {
                            if area.contains(point) {
                                let key = pincode.permutation[index];
                                println!("User pressed button {}", key);
                                pin_button_pushed(&key, &PIN_BUTTON_AREA[index], fast_display);
                                pincode.input(rng, key);
                                responsive = false;
                                break;
                            }
                        }
                if pincode.position == 4 {
                    if pincode.code == PIN_CODE_MOCK {
                        println!("You win");
                        *self = UIState::End;
                    } else {
                        println!("kaput");
                        *self = UIState::Locked;
                    }
                }
            },
            UIState::Locked => (),
            End => (),
        }
        Ok(responsive)
    } 
}



fn main() {
    
    // Prepare
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(264, 176));

    let mut rng = thread_rng();

    /*
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    let border_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(3)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
    let textbox_style = TextBoxStyleBuilder::new()
            .height_mode(HeightMode::FitToText)
            .alignment(HorizontalAlignment::Justified)
            .paragraph_spacing(5)
            .build();
    */
    //let bounds = Rectangle::new(Point::zero(), Size::new(176, 0));


    //Rectangle::new(Point::new(52, 15), Size::new(16, 16))
    //    .into_styled(fill)
    //    .draw(&mut display);

    //TextBox::with_textbox_style("fuck", bounds, character_style, textbox_style).draw(&mut display);

    let fill = PrimitiveStyle::with_fill(BinaryColor::Off);



    let mut state = UIState::new(&mut rng);

    // Layout

    

    // Draw
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Inverted)
        .build();
    let mut window = Window::new("Hello world", &output_settings);//.show_static(&display);
    let mut responsive = false;
    let mut update_started = Instant::now();

    // event loop:
    //
    // 1. draw
    // 2. collect input
    // 3. handle input
    // 4. do internal things
    loop {
        // display event; it would be delayed
        if !responsive {
            //sleep(Duration::new(1, 0));
            if update_started.elapsed().cmp(&SLOW_UPDATE_TIME).is_gt() {
                display.bounding_box().into_styled(fill).draw(&mut display);
                    match state {
                        UIState::PinEntry(ref pin) => {
                            match pinpad(&pin.permutation, &mut display) {
                                Ok(()) => (),
                                Err(e) => println!("{e}"),
                            };
                        },
                        _ => {},
                    }
                responsive = true;
            }
        }

        window.update(&display);
        for event in window.events() {
            //heavy shitter
            //println!("{:?}", event);
            match event {
                SimulatorEvent::MouseButtonDown{mouse_btn: _, point: point} => {
                    if responsive {
                        println!("{}", point);
                        match state.handle_event(point, &mut rng, &mut display) {
                            Ok(a) => responsive = a,
                            Err(e) => println!("{e}"),
                        };
                        if !responsive {
                            update_started = Instant::now();
                        }
                    }
                },
                SimulatorEvent::Quit => return,
                _ => (),
            }
        };

    }
}

