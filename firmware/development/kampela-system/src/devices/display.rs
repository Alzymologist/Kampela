//! display control functions

use efm32pg23_fix::Peripherals;
use crate::visible_delay;
use crate::peripherals::usart::*;
use crate::peripherals::gpio_pins::*;

pub const X_SIZE: usize = 176;
pub const Y_SIZE: usize = 264;


/// Send command to EPD
pub fn epaper_write_command(peripherals: &mut Peripherals, command_set: &[u8]) {
    // CS clear corresponds to selected chip, see epaper docs
    display_chip_select_set(peripherals);
    display_chip_select_clear(peripherals); // not necessary if state is known and default at start

    display_data_command_clear(peripherals);
    for command in command_set.iter() {
        write_to_usart(peripherals, *command);
    }
    display_chip_select_set(peripherals);
}

/// Send data to EPD
pub fn epaper_write_data(peripherals: &mut Peripherals, data_set: &[u8]) {
    display_chip_select_set(peripherals);
    display_chip_select_clear(peripherals); // not necessary if state is known and default at start

    display_data_command_set(peripherals);
    for data in data_set.iter() {
        write_to_usart(peripherals, *data);
    }
    display_chip_select_set(peripherals);
    //    display_data_command_clear(peripherals);
}

/// BUSY is on port B, pin [`SPI_BUSY_PIN`].
pub fn display_is_busy(peripherals: &mut Peripherals) -> bool {
    let portb_din_bits = peripherals.GPIO_S.portb_din.read().din().bits();
    portb_din_bits & (1 << SPI_BUSY_PIN) == (1 << SPI_BUSY_PIN)
}

/// Reset EPD, should be performed in many situations
///
/// Why these specific numbers for delays?
pub fn epaper_reset(peripherals: &mut Peripherals) {
    visible_delay(1);
    display_res_clear(peripherals);
    visible_delay(5);
    display_res_set(peripherals);
    visible_delay(10);
    display_res_clear(peripherals);
    visible_delay(5);
    display_chip_select_set(peripherals); // this is not the default state, should not be here
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

