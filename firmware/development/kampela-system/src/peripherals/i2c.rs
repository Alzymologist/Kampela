
use efm32pg23_fix::Peripherals;
use crate::peripherals::gpio_pins::*;
use crate::visible_delay;

#[derive(Debug)]
pub enum I2CError {
    ArbitrationLost,
    BusError,
    TransferNack,
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
    visible_delay(10);
    peripherals
        .I2C0_S
        .ctrl
        .write(|w_reg| w_reg.corerst().disable());
    visible_delay(10);

}

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



