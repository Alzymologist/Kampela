//! Power measurement unit

use efm32pg23_fix::Peripherals;

use crate::peripherals::adc;

const FAST_REFRESH_POWER: i32 = 5000;
const FULL_REFRESH_POWER: i32 = 5000;

/// Measure voltage
pub fn measure_voltage(peripherals: &mut Peripherals) -> i32 {
    adc::reset_int_flags(peripherals);
    /*
    while adc::read_int_flag(peripherals) {
        let _ = adc::read_adc(peripherals);
        adc::reset_int_flags(peripherals);
    }*/
    adc::request_adc_measure(peripherals);
    while !adc::read_int_flag(peripherals) {}
    let data = adc::read_adc(peripherals);
    
    let out = data as f32 * 0.02110;

    adc::reset_int_flags(peripherals);
    out as i32
}

/// Stops until power is sufficient for display refresh
pub fn halt_for_display_power(peripherals: &mut Peripherals) {
    while measure_voltage(peripherals) < 5000 {}
}

pub fn check_fast_display_power(peripherals: &mut Peripherals) -> bool {
    measure_voltage(peripherals) > FAST_REFRESH_POWER
}

pub fn check_full_display_power(peripherals: &mut Peripherals) -> bool {
    measure_voltage(peripherals) > FULL_REFRESH_POWER
}
