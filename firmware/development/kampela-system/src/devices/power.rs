//! Power measurement unit

use crate::{if_in_free, parallel::Operation, peripherals::adc};


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

