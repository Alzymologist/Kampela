//! all low level usart operations

use efm32pg23_fix::Peripherals;

pub const BAUDRATE_USART: u32 = 10_000_000;

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

/// Initialize USART
///
/// Assumes that clocks are enabled
pub fn usart_init(peripherals: &mut Peripherals) {
    
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

