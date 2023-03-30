//! display control functions

use efm32pg23_fix::{GPIO_S, Peripherals};
use crate::visible_delay;
use crate::peripherals::usart::*;
use crate::peripherals::gpio_pins::{display_res_clear, display_res_set};

pub const X_SIZE: usize = 176;
pub const Y_SIZE: usize = 264;


/// Send command to EPD
pub fn epaper_write_command(peripherals: &mut Peripherals, command_set: &[u8]) {
    // CS clear corresponds to selected chip, see epaper docs

    deselect_display(&mut peripherals.GPIO_S);
    select_display(&mut peripherals.GPIO_S); // not necessary if state is known and default at start
    
    display_select_command(&mut peripherals.GPIO_S);
    for command in command_set.iter() {
        write_to_usart(peripherals, *command);
    }
    deselect_display(&mut peripherals.GPIO_S);
}

/// Send data to EPD
pub fn epaper_write_data(peripherals: &mut Peripherals, data_set: &[u8]) {
    deselect_display(&mut peripherals.GPIO_S);
    select_display(&mut peripherals.GPIO_S); // not necessary if state is known and default at start

    display_select_data(&mut peripherals.GPIO_S);
    for data in data_set.iter() {
        write_to_usart(peripherals, *data);
    }
    deselect_display(&mut peripherals.GPIO_S);
    //    display_data_command_clear(peripherals);
}

/// BUSY is on port B, pin [`SPI_BUSY_PIN`].
pub fn display_is_busy(peripherals: &mut Peripherals) -> bool {
    spi_is_busy(&mut peripherals.GPIO_S)
}

/// Reset EPD, should be performed in many situations
///
/// Why these specific numbers for delays?
pub fn epaper_reset(gpio: &mut GPIO_S) {
    visible_delay(1);
    display_res_clear(gpio);
    visible_delay(5);
    display_res_set(gpio);
    visible_delay(10);
    display_res_clear(gpio);
    visible_delay(5);
    deselect_display(gpio); // this is not the default state, should not be here
    visible_delay(5);
}

/*
pub fn epaper_hw_init_fast(peripherals: &mut Peripherals) {
    epaper_hw_init(peripherals);

    epaper_write_command(peripherals, &[0x18]); // from manual, temperature sensor read
    epaper_write_data(peripherals, &[0x80]); // ?

    epaper_write_command(peripherals, &[0x22]); // from manual, Y: "load temperature value"
    epaper_write_data(peripherals, &[0xB1]); // ?
    epaper_write_command(peripherals, &[0x20]); // from manual
    while display_is_busy(peripherals) {}

    epaper_write_command(peripherals, &[0x1A]); // Y: "Write to temperature register"?
    epaper_write_data(peripherals, &[0x64, 0x00]); // ?

    epaper_write_command(peripherals, &[0x22]); // from manual, Y: "load temperature value" - again?
    epaper_write_data(peripherals, &[0x91]); // ?
    epaper_write_command(peripherals, &[0x20]); // from manual - again?
    while display_is_busy(peripherals) {}
}
*/


/// Last command in drawing protocol; actually starts display action
pub fn epaper_update(peripherals: &mut Peripherals) {
    epaper_write_command(peripherals, &[0x12]);
    visible_delay(100);
    while display_is_busy(peripherals) {}
    epaper_write_command(peripherals, &[0x22]); // from manual, Y: "Display Update Control"
    epaper_write_data(peripherals, &[0xF7]); // ?
    epaper_write_command(peripherals, &[0x20]); // from manual, Y: "Activate Display Update Sequence"
    while display_is_busy(peripherals) {}
}

/// Faster version of display action initiation
pub fn epaper_update_fast(peripherals: &mut Peripherals) {
    epaper_write_command(peripherals, &[0x12]);
    visible_delay(100);
    while display_is_busy(peripherals) {}
    epaper_write_command(peripherals, &[0x22]); // from manual, Y: "Display Update Control"
    epaper_write_data(peripherals, &[0xC7]); // ?
    epaper_write_command(peripherals, &[0x20]); // from manual, Y: "Activate Display Update Sequence"
    visible_delay(1); // why delay, from where the number?
    while display_is_busy(peripherals) {}
}

/// Partial display update; used to initiate display action when performing fast drawing without
/// full clear
pub fn epaper_update_part(peripherals: &mut Peripherals) {
    epaper_write_command(peripherals, &[0x22]); // from manual, Y: "Display Update Control"
    epaper_write_data(peripherals, &[0xFF]); // ?
    epaper_write_command(peripherals, &[0x20]); // from manual, Y: "Activate Display Update Sequence"
    visible_delay(1); // why delay, from where the number?
    while display_is_busy(peripherals) {}
}

