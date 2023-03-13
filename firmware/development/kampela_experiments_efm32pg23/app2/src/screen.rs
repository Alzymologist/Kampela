use alloc::{format, vec::Vec};

use blake2_rfc::blake2b::Blake2b;

use efm32pg23_fix::Peripherals;

use crate::visible_delay;
use crate::draw::{make_text, highlight_point};

pub fn usart_on(peripherals: &mut Peripherals) {
    peripherals
        .CMU_S
        .clken0
        .write(|w_reg| w_reg.usart0().set_bit())
}

pub fn usart_off(peripherals: &mut Peripherals) {
    peripherals
        .CMU_S
        .clken0
        .write(|w_reg| w_reg.usart0().clear_bit())
}

pub const FLASH_CS_PIN: u8 = 0;
pub const DISP_CS_PIN: u8 = 2;
pub const DISP_DC_PIN: u8 = 3;
pub const DISP_RES_PIN: u8 = 6;
pub const POW_PIN: u8 = 9;
pub const E_MISO_PIN: u8 = 1;
pub const E_MOSI_PIN: u8 = 2;
pub const E_SCK_PIN: u8 = 3;

pub const TOUCH_INT_PIN: u8 = 1;

/// Macro to switch a specific pin on a specific port.
///
/// At this point GPIO must be already clocked from elsewhere and port must be
/// in correct mode.
///
/// This does not change previously set bits.
macro_rules! gpio_pin {
    ($(#[$attr_set:meta] #[$attr_clear:meta] #[$attr_common:meta] $func_set: ident, $func_clear: ident, $port: tt, $pin: tt), *) => {
        $(
            #[$attr_set]
            #[$attr_common]
            pub fn $func_set(peripherals: &mut Peripherals) {
                peripherals
                    .GPIO_S
                    .$port
                    .modify(|r, w| w.dout().variant(r.dout().bits() | (1 << $pin)));
            }

            #[$attr_clear]
            #[$attr_common]
            pub fn $func_clear(peripherals: &mut Peripherals) {
                peripherals
                    .GPIO_S
                    .$port
                    .modify(|r, w| w.dout().variant(r.dout().bits() ^ (1 << $pin)));
            }
        )*
    }
}

gpio_pin!(
    /// Set flash chip select:
    /// Clear flash chip select:
    /// port C, pin [`FLASH_CS_PIN`].
    flash_chip_select_set,
    flash_chip_select_clear,
    portc_dout,
    FLASH_CS_PIN
);

gpio_pin!(
    /// Set display chip select:
    /// Clear display chip select:
    /// port D, pin [`DISP_CS_PIN`].
    display_chip_select_set,
    display_chip_select_clear,
    portd_dout,
    DISP_CS_PIN
);

gpio_pin!(
    /// Set display data/command:
    /// Clear display data/command:
    /// port D, pin [`DISP_DC_PIN`].
    display_data_command_set,
    display_data_command_clear,
    portd_dout,
    DISP_DC_PIN
);

gpio_pin!(
    /// Set display reset:
    /// Clear display reset:
    /// port A, pin [`DISP_RES_PIN`].
    display_res_set,
    display_res_clear,
    porta_dout,
    DISP_RES_PIN
);

pub const I2C_PIN: u8 = 4;

gpio_pin!(
    /// i2c set:
    /// i2c clear:
    /// port A, pin [`I2C_PIN`].
    i2c_set,
    i2c_clear,
    porta_dout,
    I2C_PIN
);

pub const SCL_PIN: u8 = 3;

gpio_pin!(
    /// scl set:
    /// scl clear:
    /// port A, pin [`SCL_PIN`].
    scl_set,
    scl_clear,
    porta_dout,
    SCL_PIN
);

pub const SDA_PIN: u8 = 5;

gpio_pin!(
    /// sda set:
    /// sda clear:
    /// port A, pin [`SDA_PIN`].
    sda_set,
    sda_clear,
    porta_dout,
    SDA_PIN
);

gpio_pin!(
    /// Set power:
    /// Clear power:
    /// port A, pin [`POW_PIN`].
    pow_set,
    pow_clear,
    porta_dout,
    POW_PIN
);

gpio_pin!(
    /// Set MISO:
    /// Clear MISO:
    /// port A, pin [`E_MISO_PIN`].
    miso_set,
    miso_clear,
    portc_dout,
    E_MISO_PIN
);

gpio_pin!(
    /// Set MOSI:
    /// Clear MOSI:
    /// port A, pin [`E_MOSI_PIN`].
    mosi_set,
    mosi_clear,
    portc_dout,
    E_MOSI_PIN
);

gpio_pin!(
    /// Set SCK:
    /// Clear SCK:
    /// port A, pin [`E_SCK_PIN`].
    sck_set,
    sck_clear,
    portc_dout,
    E_SCK_PIN
);

pub fn epaper_write_command(peripherals: &mut Peripherals, command_set: &[u8]) {
    // CS clear corresponds to selected chip, see epaper docs
    display_chip_select_set(peripherals);
    display_chip_select_clear(peripherals); // not necessary if state is known and default at start

    display_data_command_clear(peripherals);
    for command in command_set.iter() {
        write_to_usart(peripherals, *command);
    }
    display_chip_select_set(peripherals);
}

pub fn epaper_write_data(peripherals: &mut Peripherals, data_set: &[u8]) {
    display_chip_select_set(peripherals);
    display_chip_select_clear(peripherals); // not necessary if state is known and default at start

    display_data_command_set(peripherals);
    for data in data_set.iter() {
        write_to_usart(peripherals, *data);
    }
    display_chip_select_set(peripherals);
    //    display_data_command_clear(peripherals);
}

pub const SPI_BUSY_PIN: u8 = 4;

/// BUSY is on port B, pin [`SPI_BUSY_PIN`].
pub fn display_is_busy(peripherals: &mut Peripherals) -> bool {
    let portb_din_bits = peripherals.GPIO_S.portb_din.read().din().bits();
    portb_din_bits & (1 << SPI_BUSY_PIN) == (1 << SPI_BUSY_PIN)
}

/// Why these specific numbers for delays?
pub fn epaper_reset(peripherals: &mut Peripherals) {
    visible_delay(1);
    display_res_clear(peripherals);
    visible_delay(5);
    display_res_set(peripherals);
    visible_delay(10);
    display_res_clear(peripherals);
    visible_delay(5);
    display_chip_select_set(peripherals); // this is not the default state, should not be here
    visible_delay(5);
}

pub fn epaper_hw_init(peripherals: &mut Peripherals) {
    epaper_reset(peripherals);
    while display_is_busy(peripherals) {}
    epaper_write_command(peripherals, &[0x12]);
    visible_delay(10);
    while display_is_busy(peripherals) {}
}
/*
pub fn epaper_hw_init_fast(peripherals: &mut Peripherals) {
    epaper_hw_init(peripherals);

    epaper_write_command(peripherals, &[0x18]); // from manual, temperature sensor read
    epaper_write_data(peripherals, &[0x80]); // ?

    epaper_write_command(peripherals, &[0x22]); // from manual, Y: "load temperature value"
    epaper_write_data(peripherals, &[0xB1]); // ?
    epaper_write_command(peripherals, &[0x20]); // from manual
    while display_is_busy(peripherals) {}

    epaper_write_command(peripherals, &[0x1A]); // Y: "Write to temperature register"?
    epaper_write_data(peripherals, &[0x64, 0x00]); // ?

    epaper_write_command(peripherals, &[0x22]); // from manual, Y: "load temperature value" - again?
    epaper_write_data(peripherals, &[0x91]); // ?
    epaper_write_command(peripherals, &[0x20]); // from manual - again?
    while display_is_busy(peripherals) {}
}
*/
pub fn epaper_deep_sleep(peripherals: &mut Peripherals) {
    epaper_write_command(peripherals, &[0x10]); // from manual, enter deep sleep
    epaper_write_data(peripherals, &[0x01]); // ?
    visible_delay(100); // why delay, from where the number?
}

pub fn epaper_update(peripherals: &mut Peripherals) {
    epaper_write_command(peripherals, &[0x12]);
    visible_delay(100);
    epaper_write_command(peripherals, &[0x22]); // from manual, Y: "Display Update Control"
    epaper_write_data(peripherals, &[0xF7]); // ?
    epaper_write_command(peripherals, &[0x20]); // from manual, Y: "Activate Display Update Sequence"
    while display_is_busy(peripherals) {}
}
/*
pub fn epaper_update_fast(peripherals: &mut Peripherals) {
    epaper_write_command(peripherals, &[0x22]); // from manual, Y: "Display Update Control"
    epaper_write_data(peripherals, &[0xC7]); // ?
    epaper_write_command(peripherals, &[0x20]); // from manual, Y: "Activate Display Update Sequence"
    visible_delay(1); // why delay, from where the number?
    while display_is_busy(peripherals) {}
}

pub fn epaper_update_part(peripherals: &mut Peripherals) {
    epaper_write_command(peripherals, &[0x22]); // from manual, Y: "Display Update Control"
    epaper_write_data(peripherals, &[0xFF]); // ?
    epaper_write_command(peripherals, &[0x20]); // from manual, Y: "Activate Display Update Sequence"
    visible_delay(1); // why delay, from where the number?
    while display_is_busy(peripherals) {}
}

pub const X_SIZE: usize = 176;
pub const Y_SIZE: usize = 264;
*/
pub const BUFSIZE: usize = 5808;

pub fn epaper_draw_stuff(peripherals: &mut Peripherals, stuff: [u8; BUFSIZE]) {
    epaper_write_command(peripherals, &[0x24]); // from manual, Y: "Write Black and White image to RAM"
    epaper_write_data(peripherals, &stuff);
    epaper_update(peripherals);
}

pub fn epaper_draw_stuff_differently(peripherals: &mut Peripherals, stuff: [u8; BUFSIZE]) {
    epaper_write_command(peripherals, &[0x4E]);
    epaper_write_data(peripherals, &[0x00]);
    epaper_write_command(peripherals, &[0x4F]);
    epaper_write_data(peripherals, &[0x00]);
    epaper_write_command(peripherals, &[0x24]); // from manual, Y: "Write Black and White image to RAM"
    epaper_write_data(peripherals, &stuff);
    epaper_write_command(peripherals, &[0x26]);
    epaper_write_data(peripherals, &stuff);
    epaper_update(peripherals);
}

pub const LOGO: &[u8] = &[
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xfc, 0x30, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf1, 0xff, 0x3f, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xfc, 0x3f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc3, 0xfc, 0xff, 0xff, 0xef,
    0xff, 0xcf, 0xff, 0xff, 0xf9, 0xff, 0xff, 0xff, 0xfc, 0x3f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xc3, 0xfc, 0x7f, 0xff, 0x88, 0x00, 0x47, 0xff, 0xff, 0xf8, 0xff, 0xff, 0xff, 0xfc, 0x7f,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xc7, 0xfc, 0x3f, 0xff, 0x00, 0x00, 0x13, 0xcf, 0xff,
    0xf9, 0xff, 0xff, 0xff, 0xfc, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xe7, 0xfc, 0x3f,
    0xff, 0x7f, 0xc7, 0xf9, 0xcf, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0x7f, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xe7, 0xfc, 0x1f, 0xfe, 0x7f, 0xc7, 0xf9, 0xcf, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xfe, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xe3, 0xfc, 0x5f, 0xfe, 0x7f, 0x97, 0xfd,
    0xc7, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfe, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xe3,
    0xfc, 0x4f, 0xfc, 0x7f, 0x97, 0xfc, 0x00, 0xe1, 0xfc, 0x3f, 0x1f, 0xe0, 0xfe, 0x3f, 0x07, 0xfe,
    0x1c, 0x21, 0xcf, 0x0e, 0xe3, 0xf3, 0xfc, 0x67, 0xfc, 0x7f, 0xb7, 0xf8, 0x00, 0xcc, 0xf8, 0x38,
    0x47, 0xc4, 0x7f, 0x3e, 0x23, 0xfc, 0x89, 0x01, 0xcf, 0x0e, 0x41, 0xf3, 0xfc, 0x67, 0xfd, 0x7f,
    0x33, 0xfa, 0xe7, 0xce, 0x7c, 0xf8, 0xe3, 0x8f, 0x3f, 0x3e, 0x79, 0xf8, 0xe1, 0xc3, 0xc7, 0x1f,
    0x08, 0xf1, 0xfc, 0x73, 0xfd, 0x7f, 0x79, 0xfb, 0xe3, 0xce, 0x7c, 0x78, 0xf3, 0x8f, 0x1f, 0x3c,
    0x7c, 0xf8, 0xf1, 0xe3, 0xc7, 0x9f, 0x1c, 0xf1, 0xfc, 0x73, 0xfd, 0x7f, 0x79, 0xfb, 0xe3, 0xfe,
    0x3c, 0x7c, 0xf1, 0x8f, 0x8f, 0x1c, 0x7c, 0x7c, 0xf1, 0xe3, 0xe7, 0x9f, 0x9f, 0xf9, 0xfc, 0x79,
    0xfd, 0x7f, 0x79, 0xfb, 0xf3, 0xf0, 0x7e, 0x7c, 0xf9, 0x8f, 0x8f, 0x9c, 0x7c, 0x7c, 0xf9, 0xf3,
    0xe7, 0x8f, 0xcf, 0xf9, 0xfc, 0x01, 0xfd, 0x7f, 0x79, 0xfa, 0xf3, 0xe0, 0x7e, 0x3c, 0x78, 0x8f,
    0xf1, 0xc3, 0xfe, 0x3c, 0x78, 0x8f, 0xc7, 0x9e, 0x7e, 0x3c, 0x78, 0xf1, 0xe3, 0xcf, 0xf3, 0xf8,
    0xf8, 0xfe, 0x7c, 0x7f, 0x79, 0xf8, 0x31, 0xc7, 0x9b, 0x3e, 0x78, 0xc7, 0xc4, 0x8e, 0x3e, 0x26,
    0x78, 0xf9, 0xf3, 0xce, 0x79, 0xec, 0xf8, 0xfe, 0x7e, 0x7f, 0x79, 0xfc, 0x39, 0xe7, 0x89, 0x1e,
    0x78, 0xe7, 0xc4, 0xcf, 0x3e, 0x36, 0x7c, 0xf9, 0xf3, 0xcf, 0x3c, 0xec, 0xf8, 0xff, 0x3e, 0x7f,
    0x79, 0xf9, 0x99, 0xe7, 0xc9, 0x1e, 0x39, 0xe3, 0xce, 0xcf, 0x9e, 0x72, 0x3c, 0xf8, 0xf1, 0xcf,
    0x10, 0x64, 0xf8, 0xff, 0x3f, 0x7f, 0x79, 0xf9, 0xc9, 0xf1, 0x9c, 0x1e, 0x01, 0xf1, 0x9e, 0x4f,
    0xc4, 0x78, 0x3c, 0x78, 0xf0, 0x8f, 0x83, 0x30, 0xe0, 0x3e, 0x07, 0x3f, 0x79, 0xf9, 0xe3, 0xf8,
    0x3e, 0x3f, 0x23, 0xfc, 0x3f, 0x1f, 0xe0, 0xfc, 0x7e, 0x7c, 0xf8, 0x1f, 0xc7, 0xb1, 0xe0, 0x3e,
    0x07, 0x1f, 0x79, 0xe1, 0xff, 0xff, 0xff, 0xff, 0x3f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xf9, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x4f, 0x79, 0xc9, 0xff, 0xff, 0xff, 0xff, 0x1f, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xf8, 0xe3, 0xff, 0xff, 0xff, 0xff, 0xff, 0x63, 0x79, 0x99,
    0xff, 0xff, 0xff, 0xff, 0x9e, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xfc, 0xe3, 0xff, 0xff,
    0xff, 0xff, 0xff, 0x78, 0x78, 0x39, 0xff, 0xff, 0xff, 0xff, 0x8e, 0x7f, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xfe, 0x33, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x03, 0xff, 0xff, 0xff, 0xff,
    0xe0, 0x7f, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x07, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff,
];

pub const BAUDRATE_USART: u32 = 10_000_000;
pub const BAUDRATE_EUSART: u32 = 10_000_000;

pub fn try_this_test(peripherals: &mut Peripherals) {
    peripherals
        .CMU_S
        .clken0
        .write(|w_reg| {
            w_reg
                .gpio().set_bit()
                .hfrco0().set_bit()
                .i2c0().set_bit()
                .ldma().set_bit()
                .ldmaxbar().set_bit()
                .timer0().set_bit()
                .timer1().set_bit()
                .usart0().set_bit()
    });
    peripherals
        .CMU_S
        .clken1
        .write(|w_reg| {
            w_reg
                .eusart2().set_bit()
    });

    peripherals
        .GPIO_S
        .porta_model
        .write(|w_reg| {
            w_reg
                .mode3().wiredandpullup() // SCL for USART (display)
                .mode4().pushpull() // I2C power
                .mode5().wiredandpullup() // SDA for USART (display)
                .mode6().pushpull() // Display reset
    });
    peripherals
        .GPIO_S
        .porta_modeh
        .write(|w_reg| {
            w_reg
                .mode0().inputpullfilter() // NFC
                .mode1().pushpull() // Power 2.8 V
    });
    peripherals
        .GPIO_S
        .portb_model
        .write(|w_reg| {
            w_reg
                .mode1().input() // interrupts from display sensor
                .mode4().input() // BUSY spi
    });
    peripherals
        .GPIO_S
        .portc_model
        .write(|w_reg| {
            w_reg
                .mode0().pushpull() // Flash chip select
                .mode1().inputpull() // Display MISO
                .mode2().pushpull() // Display MOSI
                .mode3().pushpull() //Display SCK
                .mode4().pushpull() // PSRAM chip select
                .mode5().inputpull() // PSRAM MISO
                .mode6().pushpull() // PSRAM MOSI
                .mode7().pushpull() // PSRAM SCK
    });
    peripherals
        .GPIO_S
        .portd_model
        .write(|w_reg| {
            w_reg
                .mode2().inputpull() // Display chip select
                .mode3().pushpull() // Display data/command
    });
    pow_set(peripherals);
    i2c_set(peripherals);
    visible_delay(10); // wait after power set! (epaper manual for 2.8V setup)
    display_chip_select_set(peripherals);
    display_data_command_clear(peripherals);
    display_res_clear(peripherals);
    sda_set(peripherals);
    scl_set(peripherals);
    flash_chip_select_set(peripherals);
    miso_set(peripherals);
    mosi_set(peripherals);
    sck_clear(peripherals);
    psram_chip_select_set(peripherals);
    psram_miso_set(peripherals);
    psram_mosi_clear(peripherals);
    psram_sck_clear(peripherals);
    nfc_pin_clear(peripherals);

    // Setting up USART0, for epaper display

    peripherals
        .USART0_S
        .en
        .write(|w_reg| {
            w_reg
                .en().set_bit()
    });
    peripherals
        .USART0_S
        .ctrl
        .write(|w_reg| {
            w_reg
                .sync().enable()
                .clkpol().idlelow()
                .msbf().enable()
                .autotx().clear_bit()
    });
    peripherals
        .USART0_S
        .frame
        .write(|w_reg| {
            w_reg
                .databits().eight()
                .stopbits().one()
                .parity().none()
    });

    let clkdiv = ((19_000_000 - 1)/(2*BAUDRATE_USART)) << 8;

    peripherals
        .USART0_S
        .clkdiv
        .write(|w_reg| {
            w_reg
                .div().variant(clkdiv)
    });
    peripherals
        .USART0_S
        .cmd
        .write(|w_reg| {
            w_reg
                .masteren().set_bit()
                .txen().set_bit()
                .rxen().set_bit()
    });

    // display MOSI
    peripherals
        .GPIO_S
        .usart0_txroute
        .write(|w_reg| {
            w_reg
                .port().variant(2)
                .pin().variant(E_MOSI_PIN)
    });
    // display MISO
    peripherals
        .GPIO_S
        .usart0_rxroute
        .write(|w_reg| {
            w_reg
                .port().variant(2)
                .pin().variant(E_MISO_PIN)
    });
    // display SCK
    peripherals
        .GPIO_S
        .usart0_clkroute
        .write(|w_reg| {
            w_reg
                .port().variant(2)
                .pin().variant(E_SCK_PIN)
    });
    // route enable
    peripherals
        .GPIO_S
        .usart0_routeen
        .write(|w_reg| {
            w_reg
                .txpen().set_bit()
                .rxpen().set_bit()
                .clkpen().set_bit()
    });

    // setting up EUSART2, for PSRAM: why gpio setup is before init? does the order matter at all?
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

    // set up ldma
    peripherals
        .LDMA_S
        .en
        .write(|w_reg| {
            w_reg
                .en().set_bit()
    });

    peripherals
        .LDMA_S
        .ctrl
        .write(|w_reg| {
            w_reg
                .numfixed().variant(0)
    });

    peripherals
        .LDMA_S
        .synchwen
        .write(|w_reg| {
            w_reg
                .syncseten().variant(0)
                .syncclren().variant(0)
    });

    peripherals
        .LDMA_S
        .chdis
        .write(|w_reg| {
            w_reg
                .chdis().variant(0xFF)
    });

    peripherals
        .LDMA_S
        .dbghalt
        .write(|w_reg| {
            w_reg
                .dbghalt().variant(0)
    });
    
    peripherals
        .LDMA_S
        .reqdis
        .write(|w_reg| {
            w_reg
                .reqdis().variant(0)
    });

    peripherals
        .LDMA_S
        .ien
        .write(|w_reg| {
            w_reg
                .error().set_bit()
    });

    peripherals
        .LDMA_S
        .if_
        .reset();

    // SET UP NVIC for LDMA!

    // set up TIMER0 for NFC reading
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

    // set up TIMER1 for NFC reading
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

    // set up touch interrupt
    peripherals
        .GPIO_S
        .extipsell
        .write(|w_reg| w_reg.extipsel0().portb());
    peripherals
        .GPIO_S
        .extipinsell
        .write(|w_reg| w_reg.extipinsel0().pin1());
    peripherals
        .GPIO_S
        .extirise
        .write(|w_reg| w_reg.extirise().variant(0));
    peripherals
        .GPIO_S
        .extifall
        .write(|w_reg| w_reg.extifall().variant(1 << 0));
    peripherals
        .GPIO_S
        .ien
        .write(|w_reg| w_reg.extien0().set_bit());

    // SET UP NVIC for I2C0!

    peripherals
        .GPIO_S
        .i2c0_routeen
        .write(|w_reg| w_reg.sclpen().set_bit().sdapen().set_bit());
    peripherals
        .GPIO_S
        .i2c0_sdaroute
        .write(|w_reg| w_reg.port().variant(0).pin().variant(SDA_PIN));
    peripherals
        .GPIO_S
        .i2c0_sclroute
        .write(|w_reg| w_reg.port().variant(0).pin().variant(SCL_PIN));

    peripherals
        .I2C0_S
        .ien
        .reset();
    peripherals
        .I2C0_S
        .if_
        .reset();
    peripherals
        .I2C0_S
        .ctrl
        .write(|w_reg| w_reg.slave().disable().clhr().standard());
    peripherals
        .I2C0_S
        .clkdiv
        .write(|w_reg| w_reg.div().variant(12)); // divider calculated as 10, set to 12 for debug
    peripherals
        .I2C0_S
        .en
        .write(|w_reg| w_reg.en().enable());

    epaper_hw_init(peripherals);
    make_text(peripherals, LOREM_IPSUM);
    visible_delay(1000);
    epaper_deep_sleep(peripherals);

    let text = format!("psram id: {:?}", psram_read_id(peripherals));
    epaper_hw_init(peripherals);
    make_text(peripherals, &text);
    visible_delay(1000);
    epaper_deep_sleep(peripherals);

    let address = AddressPsram::new(1020u32).unwrap();
    let cap: usize = 10;
    let mut stuff: Vec<u8> = Vec::with_capacity(cap);
    for i in 0..cap {
        let a = (i%256) as u8;
        stuff.push(a);
    }
    psram_write_at_address(peripherals, address, &stuff).unwrap();
    
    let stuff1 = psram_read_at_address(peripherals, AddressPsram::new(1020u32).unwrap(), 10).unwrap();
    let stuff2 = psram_read_at_address_native(peripherals, AddressPsram::new(1020u32).unwrap(), 10);
    let stuff3 = psram_read_at_address(peripherals, AddressPsram::new(1024u32).unwrap(), 10).unwrap();

    let text = format!("{:?}\n-----\n{:?}\n-----\n{:?}", stuff1, stuff2, stuff3);
    epaper_hw_init(peripherals);
    make_text(peripherals, &text);
    visible_delay(1000);
    epaper_deep_sleep(peripherals);

/*
    let mut hasher2 = Blake2b::new(32);
    for _i in 0..100000 {
        hasher2.update(&[0]);
    }
    let hash2 = hasher2.finalize().as_bytes().to_vec();
    let text = format!("calculated looooong hash {}", hex::encode(hash2));
    epaper_hw_init(peripherals);
    make_text(peripherals, &text);
    visible_delay(1000);
    epaper_deep_sleep(peripherals);
*/


    peripherals
        .I2C0_S
        .ctrl
        .write(|w_reg| w_reg.corerst().enable());
    visible_delay(10);
    peripherals
        .I2C0_S
        .ctrl
        .write(|w_reg| w_reg.corerst().disable());
    visible_delay(10);
/*
//    ft6336_write_to(peripherals, 0xa4, 0).unwrap();
    let text = format!("got touch_id {:?}", ft6336_read_at::<1>(peripherals, 0xa4).unwrap());
    epaper_hw_init(peripherals);
    make_text(peripherals, &text);
    visible_delay(1000);
    epaper_deep_sleep(peripherals);

    loop {
        if peripherals.GPIO_S.if_.read().extif0().bit_is_set() {

            let touch_data = ft6336_read_at::<LEN_NUM_TOUCHES>(peripherals, FT6X36_REG_NUM_TOUCHES).unwrap();
            let text = format!("got touch_data {:?}", touch_data);
            epaper_hw_init(peripherals);
            make_text(peripherals, &text);
            visible_delay(1000);
            epaper_deep_sleep(peripherals);
            
            let detected_x = ((touch_data[1] as u16 & 0b00001111) << 8) | touch_data[2] as u16;
            let detected_y = ((touch_data[3] as u16 & 0b00001111) << 8) | touch_data[4] as u16;
            epaper_hw_init(peripherals);
            highlight_point(peripherals, detected_x, detected_y);
            visible_delay(1000);
            epaper_deep_sleep(peripherals);
           
            peripherals
                .GPIO_S
                .if_
                .write(|w_reg| w_reg.extif0().clear_bit())
//            break;
        }
    }

    peripherals
        .CMU_S
        .clken0
        .write(|w_reg| w_reg.gpio().clear_bit().i2c0().clear_bit().usart0().clear_bit());
*/
}

pub const LOREM_IPSUM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.";

/// Write `u8` data to usart.
///
/// At this point USART must be already clocked from elsewhere.
pub fn write_to_usart(peripherals: &mut Peripherals, data: u8) -> u8 {
    while peripherals.USART0_S.status.read().txbl().bit_is_clear() {}

    peripherals
        .USART0_S
        .txdata
        .write(|w_reg| w_reg.txdata().variant(data));

    while peripherals.USART0_S.status.read().txc().bit_is_clear() {}

    peripherals.USART0_S.rxdata.read().rxdata().bits()
}

pub fn ft6336_write_to(peripherals: &mut Peripherals, position: u8, data: u8) -> Result<(), I2CError> {
    // abort unexpected processes
    if peripherals
        .I2C0_S
        .state
        .read()
        .busy()
        .bit_is_set()
    {
        peripherals
            .I2C0_S
            .cmd
            .write(|w_reg| w_reg.abort().set_bit());
        visible_delay(10);
    }

    // clear pending commands and tx
    peripherals
        .I2C0_S
        .cmd
        .write(|w_reg| w_reg.clearpc().set_bit().cleartx().set_bit());
    visible_delay(10);

    // clear rx buffer content
    while peripherals
        .I2C0_S
        .status
        .read()
        .rxdatav()
        .bit_is_set()
    {
        let _dummy_data = peripherals
            .I2C0_S
            .rxdata
            .read()
            .bits();
        visible_delay(10);
    }
    
    // clear interrupt flags
    peripherals
        .I2C0_S
        .if_
        .reset();
    
    // enable interrupts sources
    peripherals
        .I2C0_S
        .ien
        .write(|w_reg| w_reg.nack().set_bit().ack().set_bit().mstop().set_bit().rxdatav().set_bit().arblost().set_bit().buserr().set_bit());

    // i2c transfer sequence

    check_i2c_errors(peripherals)?;
    
    // send address `0x38 << 1`, for writing data
    peripherals
        .I2C0_S
        .txdata
        .write(|w_reg| w_reg.txdata().variant(0b1110000));
    visible_delay(10);
    peripherals
        .I2C0_S
        .cmd
        .write(|w_reg| w_reg.start().set_bit());
    visible_delay(10);

    acknowledge_i2c_tx(peripherals)?;
    
    // send position, single byte
    peripherals
        .I2C0_S
        .txdata
        .write(|w_reg| w_reg.txdata().variant(position));
    visible_delay(10);

    acknowledge_i2c_tx(peripherals)?;

    // send data to record at position, single byte
    peripherals
        .I2C0_S
        .txdata
        .write(|w_reg| w_reg.txdata().variant(data));
    visible_delay(10);

    acknowledge_i2c_tx(peripherals)?;
    
    peripherals
        .I2C0_S
        .cmd
        .write(|w_reg| w_reg.stop().set_bit());
    visible_delay(10);
    
    mstop_i2c_wait_and_clear(peripherals)?;
    
    // disable interrupts sources
    peripherals
        .I2C0_S
        .ien
        .reset();
    
    Ok(())
}


pub fn ft6336_read_at<const LEN: usize>(peripherals: &mut Peripherals, position: u8) -> Result<[u8; LEN], I2CError> {
    // abort unexpected processes
    if peripherals
        .I2C0_S
        .state
        .read()
        .busy()
        .bit_is_set()
    {
        peripherals
            .I2C0_S
            .cmd
            .write(|w_reg| w_reg.abort().set_bit());
        visible_delay(10);
    }

    // clear pending commands and tx
    peripherals
        .I2C0_S
        .cmd
        .write(|w_reg| w_reg.clearpc().set_bit().cleartx().set_bit());
    visible_delay(10);

    // clear rx buffer content
    while peripherals
        .I2C0_S
        .status
        .read()
        .rxdatav()
        .bit_is_set()
    {
        let _dummy_data = peripherals
            .I2C0_S
            .rxdata
            .read()
            .bits();
        visible_delay(10);
    }
    
    // clear interrupt flags
    peripherals
        .I2C0_S
        .if_
        .reset();
    
    // enable interrupts sources
    peripherals
        .I2C0_S
        .ien
        .write(|w_reg| w_reg.nack().set_bit().ack().set_bit().mstop().set_bit().rxdatav().set_bit().arblost().set_bit().buserr().set_bit());

    // i2c transfer sequence

    check_i2c_errors(peripherals)?;
    
    // send address `0x38 << 1`, for writing data
    peripherals
        .I2C0_S
        .txdata
        .write(|w_reg| w_reg.txdata().variant(0b1110000));
    visible_delay(10);
    peripherals
        .I2C0_S
        .cmd
        .write(|w_reg| w_reg.start().set_bit());
    visible_delay(10);

    acknowledge_i2c_tx(peripherals)?;
    
    // transfer write data, single byte
    peripherals
        .I2C0_S
        .txdata
        .write(|w_reg| w_reg.txdata().variant(position));
    visible_delay(10);

    acknowledge_i2c_tx(peripherals)?;

    // send address `(0x38 << 1)|1`, for reading data
    peripherals
        .I2C0_S
        .cmd
        .write(|w_reg| w_reg.start().set_bit());
    visible_delay(10);
    peripherals
        .I2C0_S
        .txdata
        .write(|w_reg| w_reg.txdata().variant(0b1110001));
    visible_delay(10);

    acknowledge_i2c_tx(peripherals)?;
    
    let mut rx_data_collected: Vec<u8> = Vec::with_capacity(LEN);
    
    for i in 0..LEN {
        rx_data_collected.push(read_i2c_rx(peripherals)?);
        if i == LEN-1 {
            peripherals
                .I2C0_S
                .cmd
                .write(|w_reg| w_reg.nack().set_bit());
            visible_delay(10);
            peripherals
                .I2C0_S
                .cmd
                .write(|w_reg| w_reg.stop().set_bit());
            visible_delay(10);
        } else {
            peripherals
                .I2C0_S
                .cmd
                .write(|w_reg| w_reg.ack().set_bit());
            visible_delay(10);
        }
    }
    
    mstop_i2c_wait_and_clear(peripherals)?;
    
    // disable interrupts sources
    peripherals
        .I2C0_S
        .ien
        .reset();
    
    Ok(rx_data_collected.try_into().expect("constant size, always fit"))
}

pub const FT6X36_REG_CHIPID: u8 = 0xA3;
pub const LEN_CHIPID: usize = 1;

pub const FT6X36_REG_NUM_TOUCHES: u8 = 0x02;
pub const LEN_NUM_TOUCHES: usize = 5;

pub fn read_i2c_rx(peripherals: &mut Peripherals) -> Result<u8, I2CError> {
    check_i2c_errors(peripherals)?;
    while peripherals
        .I2C0_S
        .if_
        .read()
        .rxdatav()
        .bit_is_clear()
    {
        check_i2c_errors(peripherals)?;
    }
    let out = peripherals
        .I2C0_S
        .rxdata
        .read()
        .rxdata()
        .bits();
    visible_delay(10);

    // Errata I2C_E303, patch follows sdk
    if peripherals.I2C0_S.status.read().rxdatav().bit_is_clear() & peripherals.I2C0_S.status.read().rxfull().bit_is_set() {
        let _dummy_data = peripherals
            .I2C0_S
            .rxdata
            .read()
            .bits();
        visible_delay(10);
        peripherals
            .I2C0_S
            .if_
            .write(|w_reg| w_reg.rxuf().clear_bit());
    }
    
    peripherals
        .I2C0_S
        .if_
        .write(|w_reg| w_reg.rxdatav().clear_bit().rxfull().clear_bit());

    Ok(out)
}

#[derive(Debug)]
pub enum I2CError {
    ArbitrationLost,
    BusError,
    TransferNack,
}

pub fn check_i2c_errors(peripherals: &mut Peripherals) -> Result<(), I2CError> {
    let if_read = peripherals
        .I2C0_S
        .if_
        .read();
    if if_read.arblost().bit_is_set() {return Err(I2CError::ArbitrationLost)}
    if if_read.buserr().bit_is_set() {return Err(I2CError::BusError)}
    Ok(())
}

pub fn acknowledge_i2c_tx(peripherals: &mut Peripherals) -> Result<(), I2CError> {
    check_i2c_errors(peripherals)?;
    while peripherals
        .I2C0_S
        .if_
        .read()
        .ack()
        .bit_is_clear()
    {
        check_i2c_errors(peripherals)?;

        if peripherals
            .I2C0_S
            .if_
            .read()
            .nack()
            .bit_is_set()
        {
            // clear interrupt flag
            peripherals
                .I2C0_S
                .if_
                .write(|w_reg| w_reg.nack().clear_bit());
            // stop
            peripherals
                .I2C0_S
                .cmd
                .write(|w_reg| w_reg.stop().set_bit());
            visible_delay(10);
            return Err(I2CError::TransferNack)
        }
    }
    // clear interrupt flag
    peripherals
        .I2C0_S
        .if_
        .write(|w_reg| w_reg.ack().clear_bit());

    Ok(())
}

pub fn mstop_i2c_wait_and_clear(peripherals: &mut Peripherals) -> Result<(), I2CError> {
    check_i2c_errors(peripherals)?;
    while peripherals
        .I2C0_S
        .if_
        .read()
        .mstop()
        .bit_is_clear()
    {
        check_i2c_errors(peripherals)?;
    }
    peripherals
        .I2C0_S
        .if_
        .write(|w_reg| w_reg.mstop().clear_bit());
    Ok(())
}

pub const PSRAM_CS_PIN: u8 = 4; // at portC
pub const PSRAM_MISO_PIN: u8 = 5; // at portC
pub const PSRAM_MOSI_PIN: u8 = 6; // at portC
pub const PSRAM_SCK_PIN: u8 = 7; // at portC

gpio_pin!(
    /// Set PSRAM CS:
    /// Clear PSRAM CS:
    /// port C, pin [`PSRAM_CS_PIN`].
    psram_chip_select_set,
    psram_chip_select_clear,
    portc_dout,
    PSRAM_CS_PIN
);

gpio_pin!(
    /// Set PSRAM MISO:
    /// Clear PSRAM MISO:
    /// port C, pin [`PSRAM_MISO_PIN`].
    psram_miso_set,
    psram_miso_clear,
    portc_dout,
    PSRAM_MISO_PIN
);

gpio_pin!(
    /// Set PSRAM MOSI:
    /// Clear PSRAM MOSI:
    /// port C, pin [`PSRAM_MOSI_PIN`].
    psram_mosi_set,
    psram_mosi_clear,
    portc_dout,
    PSRAM_MOSI_PIN
);

gpio_pin!(
    /// Set PSRAM SCK:
    /// Clear PSRAM SCK:
    /// port C, pin [`PSRAM_SCK_PIN`].
    psram_sck_set,
    psram_sck_clear,
    portc_dout,
    PSRAM_SCK_PIN
);

pub const NFC_PIN: u8 = 8; // at portA

gpio_pin!(
    /// Set NFC pin:
    /// Clear NFC pin:
    /// port A, pin [`NFC_PIN`].
    nfc_pin_set,
    nfc_pin_clear,
    porta_dout,
    NFC_PIN
);


pub fn eusart_disable(peripherals: &mut Peripherals) {
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

pub fn eusart_enable(peripherals: &mut Peripherals) {
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

pub fn eusart_reset(peripherals: &mut Peripherals) {
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

pub fn psram_write_slice(peripherals: &mut Peripherals, slice: &[u8]) {
    for byte in slice.iter() {
        psram_write_read_byte(peripherals, *byte);
    }
}

pub fn psram_read_vec(peripherals: &mut Peripherals, len: usize) -> Vec<u8> {
    let mut out: Vec<u8> = Vec::with_capacity(len);
    for _i in 0..len {
        out.push(psram_write_read_byte(peripherals, PSRAM_DUMMY))
    }
    out
}

/// PSRAM commands from manual
pub const PSRAM_RESET_ENABLE: u8 = 0x66;
pub const PSRAM_RESET: u8 = 0x99;
pub const PSRAM_READ_ID: u8 = 0x9f;
pub const PSRAM_READ: u8 = 0x03;
pub const PSRAM_WRITE: u8 = 0x02;

pub const ID_LEN: usize = 3;
pub const ADDR_LEN: usize = 3;

#[derive(Debug, Clone, Copy)]
pub struct AddressPsram([u8; ADDR_LEN]);

impl AddressPsram {
    pub fn new(into_inner: u32) -> Result<Self, MemoryError> {
        let bytes = into_inner.to_be_bytes();
        if (bytes[0] != 0) | (bytes[1] > 0x8f) {Err(MemoryError::Overflow)}
        else {Ok(Self(bytes[1..].try_into().expect("static length, always fits")))}
    }
    pub fn inner(&self) -> [u8; ADDR_LEN] {
        self.0
    }
    pub fn as_u32(&self) -> u32 {
        let mut be_bytes = [0;4];
        be_bytes[1..].copy_from_slice(&self.inner());
        u32::from_be_bytes(be_bytes)
    }
    pub fn shift(&self, position: usize) -> Result<Self, MemoryError> {
        let new_inner = self.as_u32() + position as u32;
        Self::new(new_inner)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MemoryError {
    Overflow,
    ReadTooLarge,
    TypeInfoDamaged{id: u32},
    WriteTooLarge,
}

/// PSRAM dummy command, to send a new item in rx.
///
/// Could have switched into autotx mode instead.
pub const PSRAM_DUMMY: u8 = 0xff;

pub fn psram_reset(peripherals: &mut Peripherals) {
    psram_chip_select_set(peripherals); // deselect PSRAM
    psram_chip_select_clear(peripherals); // select PSRAM, explain why
    psram_write_read_byte(peripherals, PSRAM_RESET_ENABLE);
    psram_chip_select_set(peripherals); // deselect PSRAM
    psram_chip_select_clear(peripherals); // select PSRAM
    psram_write_read_byte(peripherals, PSRAM_RESET);
    psram_chip_select_set(peripherals); // deselect PSRAM
}

pub fn psram_read_id(peripherals: &mut Peripherals) -> [u8; ID_LEN] {
    psram_chip_select_clear(peripherals); // select PSRAM
    psram_write_read_byte(peripherals, PSRAM_READ_ID);
    psram_write_slice(peripherals, &[PSRAM_DUMMY; ADDR_LEN]);
    psram_read_vec(peripherals, ID_LEN).try_into().expect("static length, always fits")
}

/// Reads data with wrapping, i.e. when the page ends, starts to read at the
/// same page beginning.
///
/// Slice length should be checked elsewhere to be sufficiently low to read only
/// data from the address going forward.
pub fn psram_read_at_address_native(peripherals: &mut Peripherals, address: AddressPsram, len: usize) -> Vec<u8> {
    psram_reset(peripherals);
    psram_read_at_address_helper(peripherals, address, len)
}

fn psram_read_at_address_helper(peripherals: &mut Peripherals, address: AddressPsram, len: usize) -> Vec<u8> {
    psram_chip_select_clear(peripherals); // select PSRAM
    psram_write_read_byte(peripherals, PSRAM_READ);
    psram_write_slice(peripherals, &address.inner());
    let out = psram_read_vec(peripherals, len);
    psram_chip_select_set(peripherals); // deselect PSRAM
    out
}

pub fn psram_read_at_address(peripherals: &mut Peripherals, address: AddressPsram, len: usize) -> Result<Vec<u8>, MemoryError> {
    psram_reset(peripherals);
    
    let start = address.as_u32();

    if start + len as u32 > PSRAM_TOTAL_SIZE {return Err(MemoryError::ReadTooLarge)}
    let mut out = Vec::with_capacity(len);

    let space_left_on_page = PSRAM_PAGE_SIZE - start%PSRAM_PAGE_SIZE;
    if len as u32 <= space_left_on_page {
        out.extend_from_slice(&psram_read_at_address_helper(peripherals, address, len));
    }
    else {
        out.extend_from_slice(&psram_read_at_address_helper(peripherals, address, space_left_on_page as usize));
        let full_pages = (len as u32 - space_left_on_page)/PSRAM_PAGE_SIZE;
        for i in 0..full_pages {
            let full_page_start = (start/PSRAM_PAGE_SIZE + 1 + i) * PSRAM_PAGE_SIZE;
            let address = AddressPsram::new(full_page_start).expect("checked that length does not overflow");
            out.extend_from_slice(&psram_read_at_address_helper(peripherals, address, PSRAM_PAGE_SIZE as usize));
        }
        let last_page_start = (start/PSRAM_PAGE_SIZE + 1 + full_pages) * PSRAM_PAGE_SIZE;
        let address = AddressPsram::new(last_page_start).expect("checked that length does not overflow");
        let tail_len = len - (space_left_on_page + full_pages*PSRAM_PAGE_SIZE) as usize;
        out.extend_from_slice(&psram_read_at_address_helper(peripherals, address, tail_len));
    }
    Ok(out)
}

/// Writes data with wrapping, i.e. when the page ends, starts to write at the
/// same page beginning.
///
/// Slice length should be checked elsewhere to be sufficiently low to fit on
/// page without wrapping.
pub fn psram_write_at_address_native(peripherals: &mut Peripherals, address: AddressPsram, slice: &[u8]) {
    psram_reset(peripherals);
    psram_write_at_address_helper(peripherals, address, slice);
}

/// Helper function, without reset at start.
///
/// Use only as a part of function with reset.
fn psram_write_at_address_helper(peripherals: &mut Peripherals, address: AddressPsram, slice: &[u8]) {
    psram_chip_select_clear(peripherals); // select PSRAM
    psram_write_read_byte(peripherals, PSRAM_WRITE);
    psram_write_slice(peripherals, &address.inner());
    psram_write_slice(peripherals, slice);
    psram_chip_select_set(peripherals); // deselect PSRAM
}

/// Write at address seamlessly, i.e. without wrapping.
///
/// Each new byte is written to the next address.
pub fn psram_write_at_address(peripherals: &mut Peripherals, address: AddressPsram, slice: &[u8]) -> Result<(), MemoryError> {
    psram_reset(peripherals);
    
    let start = address.as_u32();
    let slice_len = slice.len() as u32;

    if start + slice_len > PSRAM_TOTAL_SIZE {return Err(MemoryError::WriteTooLarge)}

    let space_left_on_page = PSRAM_PAGE_SIZE - start%PSRAM_PAGE_SIZE;
    if slice_len <= space_left_on_page {
        psram_write_at_address_helper(peripherals, address, slice);
    }
    else {
        psram_write_at_address_helper(peripherals, address, &slice[..space_left_on_page as usize]);
        let full_pages = (slice_len - space_left_on_page)/PSRAM_PAGE_SIZE;
        for i in 0..full_pages {
            let full_page_start = (start/PSRAM_PAGE_SIZE + 1 + i) * PSRAM_PAGE_SIZE;
            let address = AddressPsram::new(full_page_start).expect("checked that length does not overflow");
            let slice_start = (space_left_on_page + i*PSRAM_PAGE_SIZE) as usize;
            let slice_end = slice_start + PSRAM_PAGE_SIZE as usize;
            psram_write_at_address_helper(peripherals, address, &slice[slice_start..slice_end]);
        }
        let last_page_start = (start/PSRAM_PAGE_SIZE + 1 + full_pages) * PSRAM_PAGE_SIZE;
        let address = AddressPsram::new(last_page_start).expect("checked that length does not overflow");
        let slice_start = (space_left_on_page + full_pages*PSRAM_PAGE_SIZE) as usize;
        psram_write_at_address_helper(peripherals, address, &slice[slice_start..]);
    }
    Ok(())
}


/// PSRAM is *paged*, with data in pages wrapped at page end.
pub const PSRAM_PAGE_SIZE: u32 = 1024;

/// PSRAM total size, 2^23 byte.
///
/// Limits maximum address available to `AddressPsram([0x8f, ff, ff])`.
pub const PSRAM_TOTAL_SIZE: u32 = 67_108_864;

pub struct PsramAccess {
    pub start_address: AddressPsram,
    pub total_len: usize,
}

use core::{any::TypeId, fmt::{Debug, Display, Formatter, Result as FmtResult}};
use alloc::{borrow::ToOwned, string::{String, ToString}};

use frame_metadata::v14::{ExtrinsicMetadata, PalletCallMetadata, PalletMetadata};
use parity_scale_codec::{Compact, Decode, DecodeAll, Encode};
use substrate_parser::{AddressableBuffer, AsMetadata, ExternalMemory, ResolveType, cards::ParsedData, compacts::find_compact, decode_all_as_type, error::{MetaVersionError, ParserError, SignableError}, special_indicators::SpecialtyPrimitive, traits::PalletCallTy};
use scale_info::{form::PortableForm, interner::UntrackedSymbol, Type};

pub struct ExternalPsram<'a> {
    pub peripherals: &'a mut Peripherals,
}

impl <'a> Debug for ExternalPsram<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "ExternalPsram")
    }
}

impl <'a> ExternalMemory for ExternalPsram<'a> {
    type ExternalMemoryError = MemoryError;
}

impl MemoryError {
    pub fn error_text(&self) -> String {
        match &self {
            MemoryError::Overflow => String::from("Attempted to read at address that does not exist."),
            MemoryError::ReadTooLarge => String::from("Attempted to read at address that does not exist."),
            MemoryError::TypeInfoDamaged{id} => format!("Type information for type id {id} in metadata is damaged."),
            MemoryError::WriteTooLarge => String::from("Attempted to write into PSRAM data that exceeds available space."),
        }
    }
}

impl Display for MemoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.error_text())
    }
}

impl <'a> AddressableBuffer<ExternalPsram<'a>> for PsramAccess {
    type ReadBuffer = Vec<u8>;
    fn total_len(&self) -> usize {
        self.total_len
    }
    fn read_slice(&self, ext_memory: &mut ExternalPsram<'a>, position: usize, len: usize) -> Result<Self::ReadBuffer, ParserError<ExternalPsram<'a>>> {
        if self.total_len() < position {return Err(ParserError::OutOfRange { position, total_length: self.total_len() })}
        if self.total_len() < (position + len) {return Err(ParserError::DataTooShort { position: self.total_len(), minimal_length: position + len - self.total_len() })}
        let address = self.start_address.shift(position).map_err(ParserError::External)?;
        psram_read_at_address(ext_memory.peripherals, address, len).map_err(ParserError::External)
    }
    fn limit_length(&self, new_len: usize) -> Self {
        if new_len > self.total_len {panic!()}
        PsramAccess {
            total_len: new_len,
            start_address: self.start_address,
        }
    }
}

#[derive(Debug)]
pub struct MetalRegistry {
    pub start_address: AddressPsram,
    pub registry: Vec<EntryPsram>,
}

#[derive(Debug)]
pub struct EntryPsram {
    pub position: usize,
    pub entry_len: usize,
}

impl <'a> ResolveType<ExternalPsram<'a>> for MetalRegistry {
    fn resolve_ty(&self, id: u32, ext_memory: &mut ExternalPsram<'a>) -> Result<Type<PortableForm>, ParserError<ExternalPsram<'a>>> {
        match self.registry.get(id as usize) {
            Some(entry_psram) => {
                let address = self.start_address.shift(entry_psram.position).map_err(ParserError::External)?;
                let encoded_type_data = psram_read_at_address(ext_memory.peripherals, address, entry_psram.entry_len).map_err(ParserError::External)?;
                Type::<PortableForm>::decode_all(&mut &encoded_type_data[..]).map_err(|_| ParserError::External(MemoryError::TypeInfoDamaged{id}))
            },
            None => Err(ParserError::V14TypeNotResolved { id }),
        }
    }
}

#[derive(Debug)]
pub struct CheckedMetadataMetal {
    pub types: MetalRegistry,
    pub pallets: Vec<PalletMetal>,
    pub extrinsic: ExtrinsicMetadata<PortableForm>,
    pub ty: UntrackedSymbol<TypeId>,
    pub version: String,
}

#[derive(Debug)]
pub struct PalletMetal {
    pub name: String,
    pub calls: Option<PalletCallMetadata<PortableForm>>,
    pub index: u8,
}

impl <'a> AsMetadata<ExternalPsram<'a>> for CheckedMetadataMetal {
    type TypeRegistry = MetalRegistry;
    fn types(&self) -> &Self::TypeRegistry {
        &self.types
    }
    fn find_calls_ty(
        &self,
        pallet_index: u8,
        ext_memory: &mut ExternalPsram<'a>,
    ) -> Result<PalletCallTy, SignableError<ExternalPsram<'a>>> {
        let mut found_calls_in_pallet: Option<UntrackedSymbol<TypeId>> = None;

        let mut found_pallet_name: Option<String> = None;
        for x in self.pallets.iter() {
            if x.index == pallet_index {
                found_pallet_name = Some(x.name.to_owned());
                if let Some(a) = &x.calls {
                    found_calls_in_pallet = Some(a.ty);
                }
                break;
            }
        }

        let pallet_name = match found_pallet_name {
            Some(a) => a,
            None => return Err(SignableError::PalletNotFound(pallet_index)),
        };

        let call_ty = match found_calls_in_pallet {
            Some(calls_in_pallet_symbol) => self
                .types
                .resolve_ty(calls_in_pallet_symbol.id(), ext_memory)
                .map_err(SignableError::Parsing)?,
            None => return Err(SignableError::NoCallsInPallet(pallet_name)),
        };

        Ok(PalletCallTy {
            pallet_name,
            call_ty,
        })
    }
    fn version_printed(&self) -> Result<String, MetaVersionError> {
        Ok(self.version.to_owned())
    }
    fn extrinsic(&self) -> ExtrinsicMetadata<PortableForm> {
        self.extrinsic.to_owned()
    }
    fn ty(&self) -> UntrackedSymbol<TypeId> {
        self.ty.to_owned()
    }
}

#[derive(Decode)]
struct Tail {
    extrinsic: ExtrinsicMetadata<PortableForm>,
    ty: UntrackedSymbol<TypeId>,
}

fn force_decode_at<T: Decode>(psram_data: &PsramAccess, ext_memory: &mut ExternalPsram<'_>, start_position: usize, err_at: ReceivedMetadataError) -> Result<(T, usize), ReceivedMetadataError> {
    let mut data = Vec::with_capacity(psram_data.total_len - start_position);
    let mut out: Option<(T, usize)> = None;
    for i in 0..psram_data.total_len - start_position {
        data.push(psram_data.read_byte(ext_memory, start_position+i).map_err(|_| err_at.to_owned())?);
        if let Ok(a) = T::decode(&mut &data[..]) {
            out = Some((a, start_position+i+1));
            break;
        }
    }
    match out {
        Some(a) => Ok(a),
        None => Err(err_at),
    }
}

impl <'a> CheckedMetadataMetal {
    /// Assume here that the metadata is received as SCALE-encoded
    /// `RuntimeMetadataV14` with known length.
    ///
    /// Provided `PsramAccess` corresponds to whole encoded metadata.
    pub fn from(psram_data: &PsramAccess, ext_memory: &mut ExternalPsram<'a>) -> Result<Self, ReceivedMetadataError> {
        let mut position = 0usize;

        // Metadata starts with types registry, a vec of Type descriptors.
        // Search for compact, the number of `PortableType` entries to follow.
        let found_compact = find_compact::<u32, PsramAccess, ExternalPsram<'a>>(psram_data, ext_memory, position).map_err(|_| ReceivedMetadataError::RegistryFormat)?;

        let types_set_len = found_compact.compact;
        let mut registry: Vec<EntryPsram> = Vec::with_capacity(types_set_len as usize);
        position = found_compact.start_next_unit;
        
        for entry_number in 0..types_set_len {
            let current_address = psram_data.start_address.shift(position).map_err(ReceivedMetadataError::Memory)?;

            // Each `PortableType` starts with compact of the entry number.
            let entry_compact_encoded_expected = Compact(entry_number).encode();
            let entry_compact_encoded_read = psram_read_at_address(ext_memory.peripherals, current_address, entry_compact_encoded_expected.len()).map_err(ReceivedMetadataError::Memory)?;
            
            if entry_compact_encoded_expected != entry_compact_encoded_read {return Err(ReceivedMetadataError::RegistryFormat)}
            position += entry_compact_encoded_expected.len();

            // And is followed by encoded `Type<PortableForm>` entry.
            let (_ty, entry_len) = force_decode_at::<Type<PortableForm>>(psram_data, ext_memory, position, ReceivedMetadataError::RegistryFormat)?;

            registry.push(EntryPsram{position, entry_len});

            position += entry_len;
        }
        let types = MetalRegistry {start_address: psram_data.start_address, registry};

        // Next, metadata contains pallet information,
        // `Vec<PalletMetadata<PortableForm>>`.
        // Search for compact, the number of `PalletMetadata<PortableForm>`
        // entries to follow.
        let found_compact = find_compact::<u32, PsramAccess, ExternalPsram<'a>>(psram_data, ext_memory, position).map_err(|_| ReceivedMetadataError::PalletsFormat)?;
        let pallets_set_len = found_compact.compact;
        let mut pallets: Vec<PalletMetal> = Vec::with_capacity(pallets_set_len as usize);
        let mut runtime_version_data_and_ty = None;
        let mut system_block = false;

        position = found_compact.start_next_unit;
        for _entry_number in 0..pallets_set_len {
            let (pallet, entry_len) = force_decode_at::<PalletMetadata<PortableForm>>(psram_data, ext_memory, position, ReceivedMetadataError::PalletsFormat)?;
            position += entry_len;
        
            if pallet.name == "System" {
                if !system_block {
                    for constant in pallet.constants.iter() {
                        if constant.name == "Version" {
                            runtime_version_data_and_ty = Some((constant.value.to_vec(), constant.ty))
                        }
                    }
                    system_block = true;
                }
                else {return Err(ReceivedMetadataError::DoubleSystemPallet)}
            }
        
            let pallet_metal = PalletMetal {
                name: pallet.name,
                calls: pallet.calls,
                index: pallet.index,
            };
            pallets.push(pallet_metal);
        }

        if !system_block {
            return Err(ReceivedMetadataError::NoSystemPallet);
        }

        let tail_data = psram_data.read_slice(ext_memory, position, psram_data.total_len-position).map_err(|_| ReceivedMetadataError::TailFormat)?;
        let tail = Tail::decode_all(&mut &tail_data[..]).map_err(|_| ReceivedMetadataError::TailFormat)?;

        let mut spec_version = None;
        match runtime_version_data_and_ty {
            Some((value, ty)) => {
                match decode_all_as_type::<&[u8], ExternalPsram<'a>, CheckedMetadataMetal>(
                    &ty,
                    &value.as_ref(),
                    ext_memory,
                    &types,
                ) {
                    Ok(extended_data) => {
                        if let ParsedData::Composite(fields) = extended_data.data {
                            for field in fields.iter() {
                                match field.data.data {
                                    ParsedData::PrimitiveU8 {
                                        value,
                                        specialty: SpecialtyPrimitive::SpecVersion,
                                    } => {
                                        spec_version = Some(value.to_string());
                                        break;
                                    }
                                    ParsedData::PrimitiveU16 {
                                        value,
                                        specialty: SpecialtyPrimitive::SpecVersion,
                                    } => {
                                        spec_version = Some(value.to_string());
                                        break;
                                    }
                                    ParsedData::PrimitiveU32 {
                                        value,
                                        specialty: SpecialtyPrimitive::SpecVersion,
                                    } => {
                                        spec_version = Some(value.to_string());
                                        break;
                                    }
                                    ParsedData::PrimitiveU64 {
                                        value,
                                        specialty: SpecialtyPrimitive::SpecVersion,
                                    } => {
                                        spec_version = Some(value.to_string());
                                        break;
                                    }
                                    ParsedData::PrimitiveU128 {
                                        value,
                                        specialty: SpecialtyPrimitive::SpecVersion,
                                    } => {
                                        spec_version = Some(value.to_string());
                                        break;
                                    }
                                    _ => (),
                                }
                            }
                        } else {
                            return Err(ReceivedMetadataError::UnexpectedRuntimeVersionFormat);
                        }
                    }
                    Err(_) => return Err(ReceivedMetadataError::RuntimeVersionNotDecodeable),
                }
            }
            None => return Err(ReceivedMetadataError::NoVersionInConstants),
        }
        let version = match spec_version {
            Some(a) => a,
            None => {return Err(ReceivedMetadataError::NoSpecVersionIdentifier)},
        };

        Ok(CheckedMetadataMetal{
            types,
            pallets,
            extrinsic: tail.extrinsic,
            ty: tail.ty,
            version,
        })

    }
}

#[derive(Clone, Debug)]
pub enum ReceivedMetadataError {
    DoubleSystemPallet,
    Memory(MemoryError),
    MetadataStructure,
    NoSpecVersionIdentifier,
    NoSystemPallet,
    NoVersionInConstants,
    PalletsFormat,
    RegistryFormat,
    RuntimeVersionNotDecodeable,
    TailFormat,
    UnableToDecode,
    UnexpectedRuntimeVersionFormat
}

