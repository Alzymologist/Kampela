
use efm32pg23_fix::Peripherals;

/// Set up LDMA for NFC capture
pub fn init_ldma(peripherals: &mut Peripherals) {
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

}
