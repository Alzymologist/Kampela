//! Power measurement unit

use efm32pg23_fix::Peripherals;

use crate::{in_free, if_in_free, parallel::Operation, peripherals::adc};


pub struct ADC {
    state: ADCState,
    last_value: i32,
}

pub enum ADCState {
    Ready,
    Request,
}

impl ADC {
    pub fn read(&self) -> i32 {
        self.last_value * 211 / 10000
    }
}

impl Operation for ADC {
    type Input<'a> = ();
    type Output = ();
    type StateEnum = ADCState;

    fn new() -> Self {
        Self{
            state: ADCState::Ready,
            last_value: 0,
        }
    }

    fn wind(&mut self, state: ADCState, _delay: usize) {
        self.state = state;
    }

    fn advance(&mut self, _: Self::Input<'_>) {
        match self.state {
            ADCState::Ready => {
                adc::reset_int_flags();
                adc::request_adc_measure();
                self.state = ADCState::Request;
            },
            ADCState::Request => {
                if if_in_free(|peripherals| adc::read_int_flag(peripherals)) == Ok(true) {
                    self.last_value = adc::read_adc();
                    self.state = ADCState::Ready;
                }
            },
        }
    }
}

/*
/// Measure voltage
///
/// TODO: place this in background
pub fn measure_voltage() -> i32 {
    let mut out = 0f32;
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
    
    out = data as f32 * 0.02110;

    adc::reset_int_flags(peripherals);
    });
    out as i32
}

pub fn check_fast_display_power() -> bool {
    return true;
    measure_voltage() > FAST_REFRESH_POWER
}

pub fn check_full_display_power() -> bool {
    return true;
    let v = measure_voltage();
    if v > FULL_REFRESH_POWER { true } else { panic!("voltage: {}", v) }
}
*/
