//! Initializations for ADC module

use efm32pg23_fix::Peripherals;

//debug tools
//TODO: remove!
use crate::draw::burning_tank;
use alloc::format;

/// Initialize ADC
///
/// assumes that CMU clock is enabled and does not check it
pub fn init_adc(peripherals: &mut Peripherals) {
    // IADC_reset()
    reset_adc(peripherals);

    //CMU clockselectset
    peripherals
        .CMU_S
        .iadcclkctrl
        .write(|w_reg| w_reg.clksel().fsrco());

    disable_adc(peripherals);

    // actually init
    peripherals
        .IADC0_S
        .ctrl
        .write(|w_reg| {
            w_reg
                .adcclksuspend0().prswudis()
                .adcclksuspend1().prswudis()
                .dbghalt().normal()
                .warmupmode().keepwarm()
                .timebase().variant(18)
                .hsclkrate().div1()
        });

    peripherals
        .IADC0_S
        .timer
        .write(|w_reg| w_reg.timer().variant(0));

    peripherals
        .IADC0_S
        .cmpthr
        .reset();

    peripherals
        .IADC0_S
        .cfg0
        .write(|w_reg| {
            w_reg
                .adcmode().normal()
                .osrhs().hispd32()
                .osrha().hiacc92()
                .analoggain().anagain0p5()
                .refsel().vbgr()
                .digavg().avg16()
                .twoscompl().auto()
        });

    // calibration data
    //
    // things are complicated...
    let ui_gain = peripherals
        .DEVINFO
        .iadc0gain0
        .read()
        .gaincana1()
        .bits();
    let offset1 = peripherals
        .DEVINFO
        .iadc0normaloffsetcal1
        .read()
        .offsetana3norm()
        .bits() as i16; // C reference did this
    let offset0 = peripherals
        .DEVINFO
        .iadc0normaloffsetcal0
        .read()
        .bits() as i16; // C reference did this

    // do some floating point
    let offset = offset1 / 16 + offset0;
    let offset_f = offset as f32 + 87505.45;
    let offset_f = (ui_gain as f32 / 5416.198) * offset_f - 524288.0;
    let offset = offset_f as i32;

    // mess with bytes now
    let ui_gain_sign = ui_gain & 0x8000;
    let ui_gain_value = ui_gain & 0x1FFF;
    let offset_truncated = match offset {
        i32::MIN..=-0x20001 => -0x20000,
        -0x20000..=0x1FFFF => offset,
        0x20000..=i32::MAX => 0x1FFFF,
    };

    peripherals
        .IADC0_S
        .scale0
        .write(|w_reg| {
            let prefab = w_reg
            .offset().variant(offset_truncated as u32)
            .gain13lsb().variant(ui_gain)
            .gain3msb();
            if ui_gain_sign == 0x8000 {
                prefab.gain100()
            } else {
                prefab.gain011()
            }
        });

    peripherals
        .IADC0_S
        .sched0
        .write(|w_reg| w_reg.prescale().variant(1));

    /*
    burning_tank(peripherals, format!("gain: {}; offset1: {}, offest0: {}, offset: {}", ui_gain, offset1, offset0, offset));
    loop{}
    */
}

/// Disable ADC
fn disable_adc(peripherals: &mut Peripherals) {
    while 
        peripherals
            .IADC0_S
            .status
            .read()
            .syncbusy()
            .bit_is_set()
    {}
    peripherals
        .IADC0_S
        .en
        .write(|w_reg| w_reg.en().disable());

    while
        peripherals
            .IADC0_S
            .en
            .read()
            .disabling()
            .bit_is_set()
    {}
}

/// reset IADC to settings similar to those after HW reset
fn reset_adc(peripherals: &mut Peripherals) {
    peripherals
        .IADC0_S
        .en
        .write(|w_reg| w_reg.en().enable());
    peripherals
        .IADC0_S
        .cmd
        .write(|w_reg| w_reg
            .singlestop().set_bit()
            .scanstop().set_bit()
            .timerdis().set_bit()
            );
    while
        peripherals.IADC0_S.status.read().singlequeuepending().bit_is_set() |
        peripherals.IADC0_S.status.read().scanqueuepending().bit_is_set() |
        peripherals.IADC0_S.status.read().converting().bit_is_set() |
        peripherals.IADC0_S.status.read().timeractive().bit_is_set() 
    {}
    peripherals
        .IADC0_S
        .maskreq
        .reset();
    peripherals
        .IADC0_S
        .single
        .reset();
    while
        peripherals.IADC0_S.status.read().singlewritepending().bit_is_set() |
        peripherals.IADC0_S.status.read().maskreqwritepending().bit_is_set()
        {}
    while
        peripherals.IADC0_S.status.read().singlefifodv().bit_is_set() |
        peripherals.IADC0_S.singlefifostat.read().fiforeadcnt().ne(&0)
    {
        let _dummy_data = peripherals
            .IADC0_S
            .singlefifodata
            .read()
            .data();
    }
    while
        peripherals.IADC0_S.status.read().scanfifodv().bit_is_set() |
        peripherals.IADC0_S.scanfifostat.read().fiforeadcnt().ne(&0)
    {
        let _dummy_data = peripherals
            .IADC0_S
            .scanfifodata
            .read()
            .data();
    }
    let _dummy_data = peripherals
        .IADC0_S
        .singledata
        .read()
        .data();
    let _dummy_data = peripherals
        .IADC0_S
        .scandata
        .read()
        .data();
    disable_adc(peripherals);
    peripherals
        .IADC0_S
        .ctrl
        .reset();
    peripherals
        .IADC0_S
        .timer
        .reset();
    peripherals
        .IADC0_S
        .trigger
        .reset();
    peripherals
        .IADC0_S
        .cmpthr
        .reset();
    peripherals
        .IADC0_S
        .singlefifocfg
        .reset();
    peripherals
        .IADC0_S
        .scanfifocfg
        .reset();
    peripherals
        .IADC0_S
        .cfg0
        .reset(); 
    peripherals
        .IADC0_S
        .scale0
        .reset();
    peripherals
        .IADC0_S
        .sched0
        .reset();
    peripherals
        .IADC0_S
        .cfg1
        .reset();
    peripherals
        .IADC0_S
        .scale1
        .reset();
    peripherals
        .IADC0_S
        .sched1
        .reset();

    peripherals
        .IADC0_S
        .scan0
        .reset();
    peripherals
        .IADC0_S
        .scan1
        .reset();
    peripherals
        .IADC0_S
        .scan2
        .reset();
    peripherals
        .IADC0_S
        .scan3
        .reset();
    peripherals
        .IADC0_S
        .scan4
        .reset();
    peripherals
        .IADC0_S
        .scan5
        .reset();
    peripherals
        .IADC0_S
        .scan6
        .reset();
    peripherals
        .IADC0_S
        .scan7
        .reset();
    peripherals
        .IADC0_S
        .scan8
        .reset();
    peripherals
        .IADC0_S
        .scan9
        .reset();
    peripherals
        .IADC0_S
        .scan10
        .reset();
    peripherals
        .IADC0_S
        .scan11
        .reset();
    peripherals
        .IADC0_S
        .scan12
        .reset();
    peripherals
        .IADC0_S
        .scan13
        .reset();
    peripherals
        .IADC0_S
        .scan14
        .reset();
    peripherals
        .IADC0_S
        .scan15
        .reset();

    peripherals
        .IADC0_S
        .if_
        .reset();
    peripherals
        .IADC0_S
        .ien
        .reset();
}


