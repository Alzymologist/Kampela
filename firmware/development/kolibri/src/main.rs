//! This is simulator to develop Kampela UI mocks
#![cfg(feature = "std")]
use embedded_graphics_core::{geometry::Size, pixelcolor::BinaryColor};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use rand::thread_rng;

use kampela_display_common::display_def::{SCREEN_SIZE_X, SCREEN_SIZE_Y};

use kolibri::uistate::UIState;

fn main() {
    // Prepare
    let mut display: SimulatorDisplay<BinaryColor> =
        SimulatorDisplay::new(Size::new(SCREEN_SIZE_X, SCREEN_SIZE_Y));

    let mut rng = thread_rng();

    let mut state = UIState::init(&mut rng);

    // Draw
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::Inverted)
        .build();
    let mut window = Window::new("Kolibri", &output_settings);

    let mut do_update = true;

    loop {
        if do_update {
            state.render(&mut display).unwrap();
            do_update = false;
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
                    println!("{point}");
                    do_update = state.process_touch(point, &mut rng).unwrap();
                }
                SimulatorEvent::Quit => return,
                _ => (),
            }
        }
    }
}
