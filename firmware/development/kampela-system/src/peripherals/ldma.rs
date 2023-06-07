use core::ptr::addr_of;
use cortex_m::interrupt::free;
use efm32pg23_fix::Peripherals;
use lazy_static::lazy_static;

pub const LINK_DESCRIPTORS: u32 = 0b00000111000100000111111111110000;
pub const CH_TIM0: u8 = 7;
pub const LINK_1: u32 = 0b00000000000000000000000000010011;
pub const LINK_2: u32 = 0b11111111111111111111111111010011;

pub const TIMER0_CC0_ICF: u32 = 0x40048074;

pub const BUF_QUARTER: usize = 2048;

#[repr(C)]
#[derive(Debug)]
pub struct NfcXfer {
    pub descriptors: u32,
    pub source: u32,
    pub dest: u32,
    pub link: u32,
}

#[repr(C)]
#[derive(Debug)]
pub struct NfcXferBlock {
    pub block0: NfcXfer,
    pub block1: NfcXfer,
    pub block2: NfcXfer,
    pub block3: NfcXfer,
}

/// Set up LDMA for NFC capture
pub fn init_ldma(peripherals: &mut Peripherals, nfc_descriptor_address: *const NfcXferBlock) {
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

    // start ldma transfer
    peripherals
        .LDMA_S
        .if_
        .modify(|_, w_reg| {
            w_reg
                .done7().clear_bit()
        }
    );

    peripherals
        .LDMAXBAR_S
        .ch7_reqsel
        .write(|w_reg| {
            w_reg
                .sigsel().variant(0) // _LDMAXBAR_CH_REQSEL_SIGSEL_TIMER0CC0
                .sourcesel().variant(2) // _LDMAXBAR_CH_REQSEL_SOURCESEL_TIMER0
        }
    );

    peripherals
        .LDMA_S
        .ch7_loop
        .write(|w_reg| {
            w_reg
                .loopcnt().variant(0)
        }
    );

    peripherals
        .LDMA_S
        .ch7_cfg
        .write(|w_reg| {
            w_reg
                .arbslots().one()
                .srcincsign().positive()
                .dstincsign().positive()
        }
    );
    
    peripherals
        .LDMA_S
        .ch7_link
        .write(|w_reg| {
            w_reg
                .link().clear_bit()
                .linkaddr().variant(nfc_descriptor_address as u32 >> 2)
        }
    );

    // there starts a critical section
    free(|cs| {
        peripherals
            .LDMA_S
            .ien
            .write(|w_reg| {
                w_reg
                    .chdone().variant(1 << CH_TIM0)
            }
        );

        peripherals
            .LDMA_S
            .synchwen
            .reset(); // default values, i.e. 0 for clr_off, clr_on, set_off, set_on

        peripherals
            .LDMA_S
            .chdone
            .write(|w_reg| {
                w_reg
                    .chdone7().clear_bit()
            }
        );

        peripherals
            .LDMA_S
            .linkload
            .write(|w_reg| {
                w_reg
                    .linkload().variant(1 << CH_TIM0)
            }
        );
    });
}
