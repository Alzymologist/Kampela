//! This is simulator to develop Kampela UI mocks

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
use std::time::{Duration, Instant};
use ux::u4;

#[macro_use]
extern crate lazy_static;

/// Amount of time required for full screen update; debounce
///  should be quite large as screen takes this much to clean
const SLOW_UPDATE_TIME: Duration = Duration::new(1, 0);

mod display_def;
use display_def::*;

mod pin;

mod seed_entry;

mod uistate;
use uistate::UIState;

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
                match state.render(&mut display) {
                    Ok(()) => (),
                    Err(e) => println!("{:?}", e),
                };
                responsive = true;
            }
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
                }
                SimulatorEvent::Quit => return,
                _ => (),
            }
        }

        //and here is some loop time for other things
    }
}
