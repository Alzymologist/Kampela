//! all low level usart operations

use efm32pg23_fix::Peripherals;
use crate::peripherals::gpio_pins::*;

pub const BAUDRATE_USART: u32 = 10_000_000;

/*
/// Turn USART on
pub fn usart_on(peripherals: &mut Peripherals) {
    peripherals
        .CMU_S
        .clken0
        .write(|w_reg| w_reg.usart0().set_bit())
}

/// Turn USART off
pub fn usart_off(peripherals: &mut Peripherals) {
    peripherals
        .CMU_S
        .clken0
        .write(|w_reg| w_reg.usart0().clear_bit())
}
*/

/// Initialize USART0, for EPD (display)
///
/// Assumes that clocks are enabled
pub fn init_usart(peripherals: &mut Peripherals) {
    peripherals
        .USART0_S
        .en
        .write(|w_reg| {
            w_reg
                .en().set_bit()
    });
    peripherals
        .USART0_S
        .ctrl
        .write(|w_reg| {
            w_reg
                .sync().enable()
                .clkpol().idlelow()
                .msbf().enable()
                .autotx().clear_bit()
    });
    peripherals
        .USART0_S
        .frame
        .write(|w_reg| {
            w_reg
                .databits().eight()
                .stopbits().one()
                .parity().none()
    });


    let clkdiv = ((19_000_000 - 1)/(2*BAUDRATE_USART)) << 8;

    peripherals
        .USART0_S
        .clkdiv
        .write(|w_reg| {
            w_reg
                .div().variant(clkdiv)
    });
    peripherals
        .USART0_S
        .cmd
        .write(|w_reg| {
            w_reg
                .masteren().set_bit()
                .txen().set_bit()
                .rxen().set_bit()
    });
    // display MOSI
    peripherals
        .GPIO_S
        .usart0_txroute
        .write(|w_reg| {
            w_reg
                .port().variant(2)
                .pin().variant(E_MOSI_PIN)
    });
    // display MISO
    peripherals
        .GPIO_S
        .usart0_rxroute
        .write(|w_reg| {
            w_reg
                .port().variant(2)
                .pin().variant(E_MISO_PIN)
    });
    // display SCK
    peripherals
        .GPIO_S
        .usart0_clkroute
        .write(|w_reg| {
            w_reg
                .port().variant(2)
                .pin().variant(E_SCK_PIN)
    });
    // route enable
    peripherals
        .GPIO_S
        .usart0_routeen
        .write(|w_reg| {
            w_reg
                .txpen().set_bit()
                .rxpen().set_bit()
                .clkpen().set_bit()
    });

}

/// Write `u8` data to usart.
///
/// At this point USART must be already clocked from elsewhere.
pub fn write_to_usart(peripherals: &mut Peripherals, data: u8) -> u8 {
    while peripherals.USART0_S.status.read().txbl().bit_is_clear() {}

    peripherals
        .USART0_S
        .txdata
        .write(|w_reg| w_reg.txdata().variant(data));

    while peripherals.USART0_S.status.read().txc().bit_is_clear() {}

    peripherals.USART0_S.rxdata.read().rxdata().bits()
}

