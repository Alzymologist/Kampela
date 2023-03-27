use alloc::vec::Vec;
use efm32pg23_fix::Peripherals;
use crate::visible_delay;
use crate::peripherals::i2c::{acknowledge_i2c_tx, check_i2c_errors, I2CError, mstop_i2c_wait_and_clear, read_i2c_rx};

pub const FT6X36_REG_CHIPID: u8 = 0xA3;
pub const LEN_CHIPID: usize = 1;

pub const FT6X36_REG_NUM_TOUCHES: u8 = 0x02;
pub const LEN_NUM_TOUCHES: usize = 5;

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

