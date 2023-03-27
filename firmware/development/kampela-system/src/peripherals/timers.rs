
use efm32pg23_fix::Peripherals;
use crate::peripherals::gpio_pins::*;

/// Init timers
pub fn init_timers(peripherals: &mut Peripherals) {
    init_timer0(peripherals);
    init_timer1(peripherals);
}

/// set up TIMER0 for NFC reading
fn init_timer0(peripherals: &mut Peripherals) {
    peripherals
        .GPIO_S
        .timer0_routeen
        .write(|w_reg| w_reg.cc0pen().set_bit());
    peripherals
        .GPIO_S
        .timer0_cc0route
        .write(|w_reg| {
            w_reg
                .port().variant(0)
                .pin().variant(NFC_PIN)
    });

    // synchronizing
    while peripherals.TIMER0_S.en.read().en().bit_is_set() & peripherals.TIMER0_S.status.read().syncbusy().bit_is_set() {}

    peripherals
        .TIMER0_S
        .en
        .write(|w_reg| w_reg.en().clear_bit());

    while peripherals.TIMER0_S.en.read().disabling().bit_is_set() {}

    peripherals
        .TIMER0_S
        .cc0_cfg
        .write(|w_reg| {
            w_reg
                .mode().inputcapture()
                .coist().clear_bit()
                .filt().disable()
                .insel().pin()
    });
    
    peripherals
        .TIMER0_S
        .cfg
        .write(|w_reg| {
            w_reg
                .mode().up()
                .sync().disable()
                .osmen().clear_bit()
                .qdm().x2()
                .debugrun().run()
                .dmaclract().clear_bit()
                .clksel().prescem01grpaclk()
                .dissyncout().dis()
                .ati().clear_bit()
                .presc().div1()
                
    });

    peripherals
        .TIMER0_S
        .en
        .write(|w_reg| w_reg.en().set_bit());

    peripherals
        .TIMER0_S
        .cc0_ctrl
        .write(|w_reg| {
            w_reg
                .icevctrl().falling() // everyedge
                .icedge().falling() // both
                .cufoa().none()
                .cofoa().none()
                .cmoa().none()
                .outinv().set_bit()
    });

    peripherals
        .TIMER0_S
        .cmd
        .write(|w_reg| w_reg.stop().set_bit());

    peripherals
        .TIMER0_S
        .cnt
        .reset();

    peripherals
        .TIMER0_S
        .ctrl
        .write(|w_reg| {
            w_reg
                .risea().reloadstart()
                .falla().stop() // reloadstart
                .x2cnt().clear_bit()
    });

 
}

/// set up TIMER1 for NFC reading
fn init_timer1(peripherals: &mut Peripherals) {
    peripherals
        .GPIO_S
        .timer1_routeen
        .write(|w_reg| w_reg.cc0pen().set_bit());
    peripherals
        .GPIO_S
        .timer1_cc0route
        .write(|w_reg| {
            w_reg
                .port().variant(0)
                .pin().variant(NFC_PIN)
    });
    peripherals
        .TIMER1_S
        .ien
        .write(|w_reg| w_reg.of().set_bit());

    while peripherals.TIMER1_S.en.read().en().bit_is_set() & peripherals.TIMER1_S.status.read().syncbusy().bit_is_set() {}

    peripherals
        .TIMER1_S
        .en
        .write(|w_reg| w_reg.en().clear_bit());

    while peripherals.TIMER1_S.en.read().disabling().bit_is_set() {}

    peripherals
        .TIMER1_S
        .cfg
        .write(|w_reg| {
            w_reg
                .mode().up()
                .sync().disable()
                .osmen().clear_bit()
                .qdm().x2()
                .debugrun().run()
                .dmaclract().clear_bit()
                .clksel().prescem01grpaclk()
                .dissyncout().dis()
                .ati().clear_bit()
                .presc().div128()
                
    });

    peripherals
        .TIMER1_S
        .en
        .write(|w_reg| w_reg.en().set_bit());

    peripherals
        .TIMER1_S
        .cmd
        .write(|w_reg| w_reg.stop().set_bit());

    peripherals
        .TIMER1_S
        .cnt
        .reset();

    peripherals
        .TIMER1_S
        .ctrl
        .write(|w_reg| {
            w_reg
                .risea().none()
                .falla().none()
                .x2cnt().clear_bit()
    });

    peripherals
        .TIMER1_S
        .cmd
        .write(|w_reg| w_reg.start().set_bit());

}
