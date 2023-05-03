//! Power measurement unit

use efm32pg23_fix::Peripherals;

use crate::{in_free, peripherals::adc};

const FAST_REFRESH_POWER: i32 = 5000;
const FULL_REFRESH_POWER: i32 = 5000;

/// Measure voltage
///
/// TODO: place this in background
pub fn measure_voltage() -> i32 {
    let out = 0;
    in_free(|peripherals| {
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
    });
    out as i32
}

pub fn check_fast_display_power() -> bool {
    measure_voltage() > FAST_REFRESH_POWER
}

pub fn check_full_display_power() -> bool {
    measure_voltage() > FULL_REFRESH_POWER
}
