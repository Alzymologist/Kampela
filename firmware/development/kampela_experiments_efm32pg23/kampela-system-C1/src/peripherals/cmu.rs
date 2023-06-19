//! Clock management unit initializations

use efm32pg23_fix::CMU_S;

/// Initialize all needed clock units
pub fn init_cmu(cmu: &mut CMU_S) {
    cmu
        .clken0
        .write(|w_reg| {
            w_reg
                .gpio().set_bit()
                .hfrco0().set_bit()
                .i2c0().set_bit()
                .iadc0().set_bit()
                .ldma().set_bit()
                .ldmaxbar().set_bit()
                .timer0().set_bit()
                .usart0().set_bit()
    });

    cmu
        .clken1
        .write(|w_reg| {
            w_reg
                .eusart2().set_bit()
                .semailboxhost().set_bit()
    });

}

