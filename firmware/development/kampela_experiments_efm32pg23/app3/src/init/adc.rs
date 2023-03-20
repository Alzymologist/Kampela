//! Initializations for ADC module

use efm32pg23_fix::Peripherals;

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
    let ui_gain_cana = peripherals
        .DEVINFO
        .iadc0gain0
        .read()
        .gaincana1()
        .bits();

    peripherals
        .IADC0_S
        .scale0
        .write(|w_reg| w_reg.gain13lsb().variant(ui_gain_cana));

    let offset = peripherals
        .DEVINFO
        .iadc0normaloffsetcal1
        .read()
        .offsetana3norm()
        .bits();

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


