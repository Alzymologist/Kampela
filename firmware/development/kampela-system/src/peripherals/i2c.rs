
use efm32pg23_fix::Peripherals;
use crate::peripherals::gpio_pins::*;
use cortex_m::asm::delay;
use crate::{if_in_free, in_free, FreeError};


#[derive(Debug)]
pub enum I2CError {
    ArbitrationLost,
    BusError,
    TransferNack,
    /// Peripheral mutex could not be borrowed
    PeripheralsLocked,
}

impl From<FreeError> for I2CError {
    fn from(item: FreeError) -> Self {
        Self::PeripheralsLocked
    }
}

pub fn init_i2c(peripherals: &mut Peripherals) {
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
    peripherals
        .I2C0_S
        .ctrl
        .write(|w_reg| w_reg.corerst().enable());
    delay(10000);
    peripherals
        .I2C0_S
        .ctrl
        .write(|w_reg| w_reg.corerst().disable());
    delay(100000);
}

pub fn read_i2c_rx() -> Result<u8, I2CError> {
    check_i2c_errors()?;
    while if_in_free(|peripherals|
        peripherals
            .I2C0_S
            .if_
            .read()
            .rxdatav()
            .bit_is_clear()
    )? {
        check_i2c_errors()?;
    }
    let mut out = None;
    in_free(|peripherals| 
        out = Some(peripherals
            .I2C0_S
            .rxdata
            .read()
            .rxdata()
            .bits())
    );
    delay(100000);

    // Errata I2C_E303, patch follows sdk
    if if_in_free(|peripherals| 
        peripherals
            .I2C0_S
            .status
            .read()
            .rxdatav()
            .bit_is_clear() 
        &
        peripherals
            .I2C0_S
            .status
            .read()
            .rxfull()
            .bit_is_set()
    )? {
        in_free(|peripherals| {
            let _dummy_data = peripherals
                .I2C0_S
                .rxdata
                .read()
                .bits();
            }
        );
        delay(100000);
        in_free(|peripherals| 
            peripherals
                .I2C0_S
                .if_
                .write(|w_reg| w_reg.rxuf().clear_bit())
        );
    }
    
    in_free(|peripherals|
        peripherals
            .I2C0_S
            .if_
            .write(|w_reg| w_reg.rxdatav().clear_bit().rxfull().clear_bit())
    );

    if let Some(out) = out {
        Ok(out)
    } else {
        Err(I2CError::PeripheralsLocked)
    }
}

pub fn check_i2c_errors() -> Result<(), I2CError> {
    let mut if_read = None;
    in_free(|peripherals| 
        if_read = Some(peripherals
            .I2C0_S
            .if_
            .read())
    );
    if let Some(if_read) = if_read {
        if if_read.arblost().bit_is_set() {return Err(I2CError::ArbitrationLost)}
        if if_read.buserr().bit_is_set() {return Err(I2CError::BusError)}
    } else {
        return Err(I2CError::PeripheralsLocked);
    }
    Ok(())
}

pub fn acknowledge_i2c_tx() -> Result<(), I2CError> {
    check_i2c_errors()?;
    while if_in_free(|peripherals|
        peripherals
            .I2C0_S
            .if_
            .read()
            .ack()
            .bit_is_clear()
    )? {
        check_i2c_errors()?;
        if if_in_free(|peripherals|
            peripherals
                .I2C0_S
                .if_
                .read()
                .nack()
                .bit_is_set()
        )?
        {
            // clear interrupt flag
            in_free(|peripherals| {
            peripherals
                .I2C0_S
                .if_
                .write(|w_reg| w_reg.nack().clear_bit());
            // stop
            peripherals
                .I2C0_S
                .cmd
                .write(|w_reg| w_reg.stop().set_bit());
            });
            delay(100000);
            return Err(I2CError::TransferNack)
        }
    };
    // clear interrupt flag
    in_free(|peripherals|
    peripherals
        .I2C0_S
        .if_
        .write(|w_reg| w_reg.ack().clear_bit())
    );
    Ok(())
}

pub fn mstop_i2c_wait_and_clear() -> Result<(), I2CError> {
    check_i2c_errors()?;
    while if_in_free(|peripherals|
        peripherals
            .I2C0_S
            .if_
            .read()
            .mstop()
            .bit_is_clear()
    )? {
        check_i2c_errors()?;
    }
    in_free(|peripherals|
    peripherals
        .I2C0_S
        .if_
        .write(|w_reg| w_reg.mstop().clear_bit())
    );
    Ok(())
}



