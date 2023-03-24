//! EUSART interface

use efm32pg23_fix::Peripherals;
use crate::peripherals::gpio_pins::*;

pub const BAUDRATE_EUSART: u32 = 10_000_000;

/// setting up EUSART2, for PSRAM
///
/// why gpio setup is before init? does the order matter at all?
pub fn init_eusart(peripherals: &mut Peripherals) {
    // PSRAM MOSI
    peripherals
        .GPIO_S
        .eusart2_txroute
        .write(|w_reg| {
            w_reg
                .port().variant(2)
                .pin().variant(PSRAM_MOSI_PIN)
    });
    // PSRAM MISO
    peripherals
        .GPIO_S
        .eusart2_rxroute
        .write(|w_reg| {
            w_reg
                .port().variant(2)
                .pin().variant(PSRAM_MISO_PIN)
    });
    // PSRAM SCK
    peripherals
        .GPIO_S
        .eusart2_sclkroute
        .write(|w_reg| {
            w_reg
                .port().variant(2)
                .pin().variant(PSRAM_SCK_PIN)
    });
    // route enable
    peripherals
        .GPIO_S
        .eusart2_routeen
        .write(|w_reg| {
            w_reg
                .txpen().set_bit()
                .rxpen().set_bit()
                .sclkpen().set_bit()
    });

    // EUSART2 init
    if peripherals
        .EUSART2_S
        .en
        .read()
        .bits()
        .ne(&0)
    {
        while peripherals.EUSART2_S.syncbusy.read().bits().ne(&0) {}
    }
    
    // reset EUSART
    eusart_reset(peripherals);

    // calculate clkdiv
    let clkdiv: u8 = (19_000_000/BAUDRATE_EUSART - 1).try_into().expect("BAURATE_EUSART is expected to not exceed and be comparable to reference frequency");
    
    // configure
    peripherals
        .EUSART2_S
        .cfg2
        .write(|w_reg|
            w_reg
                .master().master()
                .clkpol().idlelow()
                .clkpha().sampleleading()
                .csinv().al()
                .autotx().clear_bit()
                .autocs().set_bit()
                .clkprsen().clear_bit()
                .forceload().set_bit()
                .sdiv().variant(clkdiv)
        );
    peripherals
        .EUSART2_S
        .cfg1
        .write(|w_reg|
            w_reg
                .txfiw().oneframe()
                .rxfiw().oneframe()
        );
    peripherals
        .EUSART2_S
        .cfg0
        .write(|w_reg|
            w_reg
                .sync().sync()
                .loopbk().disable()
                .rxinv().disable()
                .txinv().disable()
                .msbf().enable()
        );
    peripherals
        .EUSART2_S
        .timingcfg
        .write(|w_reg|
            w_reg
                .cssetup().zero()
                .cshold().zero()
                .ics().zero()
                .setupwindow().variant(4)
        );
    peripherals
        .EUSART2_S
        .framecfg
        .write(|w_reg|
            w_reg
                .databits().eight()
        );
    peripherals
        .EUSART2_S
        .dtxdatcfg
        .write(|w_reg|
            w_reg
                .dtxdat().variant(0)
        );

    eusart_enable(peripherals);

    while peripherals.EUSART2_S.status.read().rxidle().bit_is_clear()
        | peripherals.EUSART2_S.status.read().txidle().bit_is_clear()  {}

    psram_reset(peripherals); // reset here, right after setup

}

fn eusart_disable(peripherals: &mut Peripherals) {
    if peripherals
        .EUSART2_S
        .en
        .read()
        .en()
        .bit_is_set() 
    {
        if peripherals.EUSART2_S.cfg0.read().sync().bit_is_clear() | peripherals.EUSART2_S.cfg2.read().master().bit_is_set() {
            // disable TX and RX
            peripherals.EUSART2_S.cmd.write(|w_reg| w_reg.rxdis().set_bit().txdis().set_bit());

            // wait for TXDIS and RXDIS to pass
            while peripherals.EUSART2_S.syncbusy.read().rxdis().bit_is_set() | peripherals.EUSART2_S.syncbusy.read().txdis().bit_is_set() {}

            // wait for TX and RX enable status to go low
            while peripherals.EUSART2_S.status.read().rxens().bit_is_set() | peripherals.EUSART2_S.status.read().txens().bit_is_set() {}
        }
        
        peripherals
            .EUSART2_S
            .en
            .write(|w_reg| w_reg.en().clear_bit());
        
        // wait for disabling to clear
        while peripherals.EUSART2_S.en.read().disabling().bit_is_set() {}
    }
}

fn eusart_enable(peripherals: &mut Peripherals) {
    peripherals
        .EUSART2_S
        .en
        .write(|w_reg| w_reg.en().set_bit());

    while peripherals.EUSART2_S.syncbusy.read().bits().ne(&0) {}

    peripherals
        .EUSART2_S
        .cmd
        .write(|w_reg| {
            w_reg
                .rxen().set_bit()
                .rxdis().clear_bit()
                .txen().set_bit()
                .txdis().clear_bit()
//                .rxblockdis().set_bit() // added
//                .rxblocken().clear_bit() // added
    });

    while peripherals.EUSART2_S.syncbusy.read().rxen().bit_is_set()
        | peripherals.EUSART2_S.syncbusy.read().rxdis().bit_is_set()
        | peripherals.EUSART2_S.syncbusy.read().txen().bit_is_set()
        | peripherals.EUSART2_S.syncbusy.read().txdis().bit_is_set()
//        | peripherals.EUSART2_S.syncbusy.read().rxblockdis().bit_is_set() // added
//        | peripherals.EUSART2_S.syncbusy.read().rxblocken().bit_is_set() // added
    {}

    while peripherals.EUSART2_S.status.read().rxens().bit_is_clear()
        | peripherals.EUSART2_S.status.read().txens().bit_is_clear()
//        | peripherals.EUSART2_S.status.read().rxblock().bit_is_set() // added
    {}
}

fn eusart_reset(peripherals: &mut Peripherals) {
    eusart_disable(peripherals);

    for _i in 0..4 {
        peripherals.EUSART2_S.cfg2.write(|w_reg| w_reg.clkpha().set_bit());
        peripherals.EUSART2_S.cfg2.write(|w_reg| w_reg.clkpha().clear_bit());
    }

    peripherals.EUSART2_S.cfg2.reset();
    peripherals.EUSART2_S.cfg1.reset();
    peripherals.EUSART2_S.cfg0.reset();
    peripherals.EUSART2_S.framecfg.reset();
    peripherals.EUSART2_S.dtxdatcfg.reset();
    peripherals.EUSART2_S.timingcfg.reset();
    peripherals.EUSART2_S.irhfcfg.reset();
    peripherals.EUSART2_S.startframecfg.reset();
    peripherals.EUSART2_S.sigframecfg.reset();
    peripherals.EUSART2_S.trigctrl.reset();
    peripherals.EUSART2_S.ien.reset();
    peripherals.EUSART2_S.if_.reset();
    peripherals.EUSART2_S.clkdiv.reset();
}

pub fn psram_reset(peripherals: &mut Peripherals) {
    psram_chip_select_set(peripherals); // deselect PSRAM
    psram_chip_select_clear(peripherals); // select PSRAM, explain why
    psram_write_read_byte(peripherals, PSRAM_RESET_ENABLE);
    psram_chip_select_set(peripherals); // deselect PSRAM
    psram_chip_select_clear(peripherals); // select PSRAM
    psram_write_read_byte(peripherals, PSRAM_RESET);
    psram_chip_select_set(peripherals); // deselect PSRAM
}

pub fn psram_write_read_byte(peripherals: &mut Peripherals, byte: u8) -> u8 {
    while peripherals.EUSART2_S.status.read().txfl().bit_is_clear() {}
    peripherals.EUSART2_S.txdata.write({|w_reg|
        w_reg
            // EUSART tx and rx are u16,
            // single byte is used here because of the commands,
            // setting used is `.databits().eight()`
            .txdata().variant(byte as u16)
    });
    while peripherals.EUSART2_S.status.read().rxfl().bit_is_clear() {}
    peripherals.EUSART2_S.rxdata.read().rxdata().bits().try_into().expect("configured frame for 8 data bits")
}

/// PSRAM commands from manual
pub const PSRAM_RESET_ENABLE: u8 = 0x66;
pub const PSRAM_RESET: u8 = 0x99;
pub const PSRAM_READ_ID: u8 = 0x9f;
pub const PSRAM_READ: u8 = 0x03;
pub const PSRAM_WRITE: u8 = 0x02;

pub const ID_LEN: usize = 3;
pub const ADDR_LEN: usize = 3;

