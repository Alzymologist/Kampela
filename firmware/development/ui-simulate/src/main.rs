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

/// Amount of time required for full screen update; debounce
///  should be quite large as screen takes this much to clean
const SLOW_UPDATE_TIME: Duration = Duration::new(1, 0);

mod pin;

use pin::Pincode;

// TODO: extract to separate module 
/// State of UI
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
                responsive = pincode.handle_event(point, rng, fast_display)?;
                match pincode.check_pin() {
                    Some(true) => {
                        println!("You win");
                        *self = UIState::End;
                    },
                    Some(false) => {
                        println!("kaput");
                        *self = UIState::Locked;
                    },
                    None => {},
                }
            },
            UIState::Locked => (),
            UIState::End => (),
        }
        Ok(responsive)
    } 
}



fn main() {
    
    // Prepare
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(264, 176));

    // TODO: rng should be generic, of course; by seeing how this breaks, find how to fix it
    let mut rng = thread_rng();

    let clear = PrimitiveStyle::with_fill(BinaryColor::Off);

    let mut state = UIState::new(&mut rng);

    // Draw
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Inverted)
        .build();
    let mut window = Window::new("Hello world", &output_settings);//.show_static(&display);
    // this variable protects the event handler from touches while screen is not yet updated;
    // debouncer with screen refresh time as debounce time
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
            if update_started.elapsed().cmp(&SLOW_UPDATE_TIME).is_gt() {
                // this unavoidable command actually takes time in real hardware
                display.bounding_box().into_styled(clear).draw(&mut display);
                match state {
                    UIState::PinEntry(ref pin) => {
                        match pin.draw(&mut display) {
                            Ok(()) => (),
                            Err(e) => println!("{e}"),
                        };
                    },
                    _ => {},
                }
                responsive = true;
            }
        }

        // this collects ui events, do not remove or simulator will crash
        window.update(&display);

        // handle input (only pushes are valid in Kampela)
        for event in window.events() {
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

        //and here is some loop time for other things
    }
}

