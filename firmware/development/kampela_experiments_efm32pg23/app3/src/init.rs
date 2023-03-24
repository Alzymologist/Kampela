use alloc::{format, vec::Vec};

use blake2_rfc::blake2b::Blake2b;

use efm32pg23_fix::Peripherals;

use crate::visible_delay;
use crate::draw::{make_text, highlight_point};
use crate::peripherals::{adc::init_adc, cmu::init_cmu};
use crate::peripherals::gpio_pins::*;
use crate::peripherals::{eusart::init_eusart, usart::init_usart};
use crate::devices::psram::psram_reset;

pub const BAUDRATE_USART: u32 = 10_000_000;

/// All peripheral initializations
pub fn init_peripherals(peripherals: &mut Peripherals) {
    // first, start clocking
    init_cmu(peripherals);

    // map GPIO pins to their functions and set their starting values
    init_gpio(peripherals);

    // Setting up USART0, for epaper display
    init_usart(peripherals);

    // Setting up EUSART2 for PSRAM
    init_eusart(peripherals);
    // ...and reset ram immediately
    psram_reset(peripherals);


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

    // set up ADC
    init_adc(peripherals);

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

    /*
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
*/
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
    */
/*
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
















            
        


