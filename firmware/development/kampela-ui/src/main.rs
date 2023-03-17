//! This is simulator to develop Kampela UI mocks
#![cfg(feature="std")]
use bitvec::prelude::{bitarr, BitArr, Msb0};
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
use std::{thread::sleep, time::{Duration, Instant}};
use ux::u4;

#[macro_use]
extern crate lazy_static;

/// Amount of time required for full screen update; debounce
///  should be quite large as screen takes this much to clean
const SLOW_UPDATE_TIME: Duration = Duration::new(1, 0);

pub mod display_def;
pub use display_def::*;

mod pin;
mod restore_or_generate;
mod seed_entry;

mod uistate;
use uistate::{UIState, UpdateRequest};

fn main() {
    // Prepare
    let mut display: SimulatorDisplay<BinaryColor> =
        SimulatorDisplay::new(Size::new(SCREEN_SIZE_X, SCREEN_SIZE_Y));

    // TODO: rng should be generic, of course; by seeing how this breaks, find how to fix it
    let mut rng = thread_rng();

    let mut state = UIState::new(&mut rng);

    // Draw
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Inverted)
        .build();
    let mut window = Window::new("Hello world", &output_settings); //.show_static(&display);
    
    let mut update = uistate::UpdateRequest::new();
    update.set_slow();

    // event loop:
    //
    // 1. draw
    // 2. collect input
    // 3. handle input
    // 4. do internal things
    loop {
        // display event; it would be delayed
        if update.read_fast() {
            window.update(&display);
            println!("skip {} events in fast update", window.events().count());
            //no-op for non-EPD
        }
        if update.read_slow() {
            match state.render(&mut display) {
                    Ok(()) => (),
                    Err(e) => println!("{:?}", e),
                };
            sleep(SLOW_UPDATE_TIME);
            window.update(&display);
            println!("skip {} events in slow update", window.events().count());
        }

        // this collects ui events, do not remove or simulator will crash
        window.update(&display);

        // handle input (only pushes are valid in Kampela)
        for event in window.events() {
            match event {
                SimulatorEvent::MouseButtonDown {
                    mouse_btn: _,
                    point,
                } => {
                    println!("{}", point);
                        match state.handle_event(point, &mut rng, &mut display) {
                            Ok(a) => update = a,
                            Err(e) => println!("{e}"),
                        };
                }
                SimulatorEvent::Quit => return,
                _ => (),
            }
        }

        //and here is some loop time for other things
    }
}
