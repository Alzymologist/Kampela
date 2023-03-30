//! Initializations for ADC module

use efm32pg23_fix::Peripherals;

/// request single ADC measurement
pub fn request_adc_measure(peripherals: &mut Peripherals) {
    peripherals
        .IADC0_S
        .cmd
        .write(|w_reg| w_reg.singlestart().set_bit());
}

/// read value from ADC
pub fn read_adc(peripherals: &mut Peripherals) -> i32 {
    let value = peripherals.IADC0_S.singledata.read().data().bits() & 0x00FFFFFF;
    (if value & 0x00800000 == 0 {
        value
    } else {
        value | 0xFF000000
    }) as i32
}

pub fn read_int_flag(peripherals: &mut Peripherals) -> bool {
    peripherals
        .IADC0_S
        .if_
        .read()
        .singledone()
        .bit()
}

pub fn reset_int_flags(peripherals: &mut Peripherals) {
    peripherals
        .IADC0_S
        .if_
        .reset();
}

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

    cfg0_set(peripherals);
    
    cfg1_set(peripherals);
    
    enable_adc(peripherals);

    init_adc_single_reader(peripherals);

    enable_adc(peripherals);

    // set gpio
    peripherals
        .GPIO_S
        .abusalloc
        .write(|w_reg| w_reg.aeven0().adc0());

    //enable interrupts
    peripherals
        .IADC0_S
        .ien
        .write(|w_reg| {
            w_reg
                .singledone().set_bit()
        });

    //request_adc_measure(peripherals);

    // TODO! remove this in prod
    //
    // This allows debugger to stay connected while Kampela sleeps in EM2 and waits for power to
    // replenish
    peripherals
        .EMU_S
        .ctrl
        .write(|w_reg| w_reg.em2dbgen().set_bit());
}

/// Calibration data for ADC defived from factory values read from memory
struct CalibrationData {
    pub offset_truncated: u32,
    pub ui_gain_sign: bool,
    pub ui_gain_value: u16,
}

impl CalibrationData {
    /// Read calibration data and convert it into ADC input.
    ///
    /// This is very hardcoded block, with only variablde `div` that differs between cfg0 and cfg1
    /// in reference code for this particular project. Could be done nicer, but we have no time for
    /// that now.
    pub fn new (peripherals: &mut Peripherals, div: i16) -> CalibrationData {
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
        let offset = offset1 / div + offset0;
        let offset_f = offset as f32 + 87505.45;
        let offset_f = (ui_gain as f32 / 5416.198) * offset_f - 524288.0;
        let offset = offset_f as i32;

        // mess with bytes now
        //
        // signed 
        let ui_gain_sign = ui_gain & 0x8000 == 0x8000;
        let ui_gain_value = ui_gain & 0x1FFF;
        let offset_truncated = match offset {
            i32::MIN..=-0x20001 => -0x20000,
            -0x20000..=0x1FFFF => offset,
            0x20000..=i32::MAX => 0x1FFFF,
        } as u32;

        CalibrationData {
            offset_truncated,
            ui_gain_sign,
            ui_gain_value,
        }
    }
}

/// Set up cfg0 for ADC.
fn cfg0_set(peripherals: &mut Peripherals) {
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
   
    let calibrations = CalibrationData::new(peripherals, 16);

    peripherals
        .IADC0_S
        .scale0
        .write(|w_reg| {
            let prefab = w_reg
            .offset().variant(calibrations.offset_truncated)
            .gain13lsb().variant(calibrations.ui_gain_value)
            .gain3msb();
            if calibrations.ui_gain_sign {
                prefab.gain100()
            } else {
                prefab.gain011()
            }
        });

    peripherals
        .IADC0_S
        .sched0
        .write(|w_reg| w_reg.prescale().variant(1));
}

/// Set up cfg1 for ADC. Not sure it is even used.
fn cfg1_set(peripherals: &mut Peripherals) {
    peripherals
        .IADC0_S
        .cfg1
        .reset();

    let calibrations = CalibrationData::new(peripherals, 4);

    peripherals
        .IADC0_S
        .scale1
        .write(|w_reg| {
            let prefab = w_reg
            .offset().variant(calibrations.offset_truncated)
            .gain13lsb().variant(calibrations.ui_gain_value)
            .gain3msb();
            if calibrations.ui_gain_sign {
                prefab.gain100()
            } else {
                prefab.gain011()
            }
        });

    peripherals
        .IADC0_S
        .sched1
        .write(|w_reg| w_reg.prescale().variant(1));
}

/// Initialize single ADC read config
fn init_adc_single_reader(peripherals: &mut Peripherals) {
    disable_adc(peripherals);
    
    peripherals
        .IADC0_S
        .singlefifocfg
        .write(|w_reg| {
            w_reg
                .alignment().right20()
                .showid().clear_bit()
                .dvl().valid1()
                .dmawufifosingle().disabled()
        });
    
    peripherals
        .IADC0_S
        .trigger
        .modify(|_, w_reg| {
            w_reg
                .singletrigsel().immediate()
                .singletrigaction().continuous()
                .singletailgate().tailgateoff()
        });

    enable_adc(peripherals);

    // measure between GND and PA0
    peripherals
        .IADC0_S
        .single
        .write(|w_reg| {
            w_reg
                .portneg().gnd()
                .pinpos().variant(0)
                .portpos().porta()
                .cfg().config0()
                .cmp().clear_bit()
        });

    disable_adc(peripherals);
}

/// Enable ADC
fn enable_adc(peripherals: &mut Peripherals) {
        peripherals
        .IADC0_S
        .en
        .write(|w_reg| w_reg.en().enable());
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
    enable_adc(peripherals);
    peripherals
        .IADC0_S
        .cmd
        .write(|w_reg| {
            w_reg
                .singlestop().set_bit()
                .scanstop().set_bit()
                .timerdis().set_bit()
        });
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


