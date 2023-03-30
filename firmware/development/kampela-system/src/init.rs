//! All initializations should be declared here when possible

use efm32pg23_fix::Peripherals;

use crate::peripherals::{
    adc::init_adc, 
    cmu::init_cmu, 
    eusart::init_eusart, 
    gpio_pins::init_gpio,
    i2c::init_i2c,
    ldma::init_ldma,
    timers::init_timers,
    usart::init_usart,
};
use crate::devices::psram::psram_reset;

/// All peripheral initializations
pub fn init_peripherals(peripherals: &mut Peripherals) {
    // first, start clocking
    init_cmu(&mut peripherals.CMU_S);

    // map GPIO pins to their functions and set their starting values
    init_gpio(&mut peripherals.GPIO_S);

    // Setting up USART0, for epaper display and flash memory
    init_usart(peripherals);

    // Setting up EUSART2 for PSRAM
    init_eusart(peripherals);
    // ...and reset ram immediately
    psram_reset(peripherals);

    // Setting up peripherals for NFC capture
    init_ldma(peripherals);
    
    // TODO: SET UP NVIC for LDMA!

    // set up TIMER0 for NFC reading
    init_timers(peripherals);

    // set up ADC to monitor power level
    init_adc(peripherals);

    // TODO: SET UP NVIC for I2C0!

    // set up i2c line to communicate with touch pad
    init_i2c(peripherals);

    // TODO: lock GPIO
}

