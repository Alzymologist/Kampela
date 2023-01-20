//! Blinking LED lights on dev board.

use efm32pg23_fix::Peripherals;

use crate::visible_delay;

pub fn gpio_on(peripherals: &mut Peripherals) {
    peripherals
        .CMU_S
        .clken0
        .write(|w_reg| w_reg.gpio().set_bit())
}

pub fn gpio_off(peripherals: &mut Peripherals) {
    peripherals
        .CMU_S
        .clken0
        .write(|w_reg| w_reg.gpio().clear_bit())
}

pub fn blink_left(peripherals: &mut Peripherals, num: u32, time1: u32, time2: u32) {
    gpio_on(peripherals);

    peripherals
        .GPIO_S
        .portc_modeh
        .write(|w_reg| w_reg.mode1().pushpull());

    for _i in 0..num {
        peripherals
            .GPIO_S
            .portc_dout
            .write(|w_reg| w_reg.dout().variant(1 << 9));

        visible_delay(time1);

        peripherals
            .GPIO_S
            .portc_dout
            .write(|w_reg| w_reg.dout().variant(0));

        visible_delay(time2);
    }

    gpio_off(peripherals);
}

pub fn blink_right(peripherals: &mut Peripherals, num: u32, time1: u32, time2: u32) {
    gpio_on(peripherals);

    peripherals
        .GPIO_S
        .portc_modeh
        .write(|w_reg| w_reg.mode0().pushpull());

    for _i in 0..num {
        peripherals
            .GPIO_S
            .portc_dout
            .write(|w_reg| w_reg.dout().variant(1 << 8));

        visible_delay(time1);

        peripherals
            .GPIO_S
            .portc_dout
            .write(|w_reg| w_reg.dout().variant(0));

        visible_delay(time2);
    }

    gpio_off(peripherals);
}
