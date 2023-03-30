//! high level functions to draw on EPD

use efm32pg23_fix::Peripherals;
use crate::visible_delay;
use crate::devices::display::*;

pub const BUFSIZE: usize = 5808;

/// Drawing protocol from some exapmle (does not work apparently)
pub fn epaper_draw_stuff(peripherals: &mut Peripherals, stuff: [u8; BUFSIZE]) {
    epaper_write_command(peripherals, &[0x24]); // from manual, Y: "Write Black and White image to RAM"
    epaper_write_data(peripherals, &stuff);
    epaper_update(peripherals);
}

/// Normal drawing protocol, with full screen clearing
pub fn epaper_draw_stuff_differently(peripherals: &mut Peripherals, stuff: [u8; BUFSIZE]) {
    epaper_reset(&mut peripherals.GPIO_S);
    epaper_write_command(peripherals, &[0x4E]);
    epaper_write_data(peripherals, &[0x00]);
    epaper_write_command(peripherals, &[0x4F]);
epaper_write_data(peripherals, &[0x07]);
    epaper_write_command(peripherals, &[0x24]); // from manual, Y: "Write Black and White image to RAM"
epaper_write_data(peripherals, &stuff);
    epaper_write_command(peripherals, &[0x26]);
    epaper_write_data(peripherals, &stuff);
    epaper_update(peripherals);
}

/// Fast and dirty refresh drawing
pub fn epaper_draw_stuff_quickly(peripherals: &mut Peripherals, stuff: [u8; BUFSIZE]) {
    epaper_reset(&mut peripherals.GPIO_S);
    epaper_write_command(peripherals, &[0x4E]);
    epaper_write_data(peripherals, &[0x00]);
    epaper_write_command(peripherals, &[0x4F]);
    epaper_write_data(peripherals, &[0x07]);
    epaper_write_command(peripherals, &[0x3C]);
    epaper_write_data(peripherals, &[0x80]);
    epaper_write_command(peripherals, &[0x24]); // from manual, Y: "Write Black and White image to RAM"
    epaper_write_data(peripherals, &stuff);
    //epaper_write_command(peripherals, &[0x26]);
    //epaper_write_data(peripherals, &stuff);
    epaper_update_part(peripherals);
}

/// Send EPD to low power state; should be performed when screen is not drawing at all times to
/// extend component life
pub fn epaper_deep_sleep(peripherals: &mut Peripherals) {
    epaper_write_command(peripherals, &[0x10]); // from manual, enter deep sleep
    epaper_write_data(peripherals, &[0x01]); // ?
    visible_delay(100); // why delay, from where the number?
}
/// EPD init, also should be performed to wake screen from sleep
pub fn epaper_hw_init(peripherals: &mut Peripherals) {
    epaper_reset(&mut peripherals.GPIO_S);
    while display_is_busy(peripherals) {}
    epaper_write_command(peripherals, &[0x12]);
    visible_delay(10);
    while display_is_busy(peripherals) {}
}

