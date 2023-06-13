//! Async touch panel operations
//!
//! This matches devices::touch blocking flow; TODO: replace that flow with this one

use crate::peripherals::i2c::{acknowledge_i2c_tx, check_i2c_errors, I2CError, mstop_i2c_wait_and_clear, ReadI2C};
use crate::parallel::{DELAY, Operation};
use crate::{in_free, if_in_free};

pub const FT6X36_REG_CHIPID: u8 = 0xA3;
pub const LEN_CHIPID: usize = 1;

pub const FT6X36_REG_NUM_TOUCHES: u8 = 0x02;
pub const LEN_NUM_TOUCHES: usize = 5;

/*
/// Blocking write function
/// TODO: replace with advance in blocking loop
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
        delay(10000);
    }

    // clear pending commands and tx
    peripherals
        .I2C0_S
        .cmd
        .write(|w_reg| w_reg.clearpc().set_bit().cleartx().set_bit());
    delay(10000);

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
        delay(10000);
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
    delay(10000);
    peripherals
        .I2C0_S
        .cmd
        .write(|w_reg| w_reg.start().set_bit());
    delay(10000);

    acknowledge_i2c_tx(peripherals)?;
    
    // send position, single byte
    peripherals
        .I2C0_S
        .txdata
        .write(|w_reg| w_reg.txdata().variant(position));
    delay(10000);

    acknowledge_i2c_tx(peripherals)?;

    // send data to record at position, single byte
    peripherals
        .I2C0_S
        .txdata
        .write(|w_reg| w_reg.txdata().variant(data));
    delay(10000);

    acknowledge_i2c_tx(peripherals)?;
    
    peripherals
        .I2C0_S
        .cmd
        .write(|w_reg| w_reg.stop().set_bit());
    delay(10000);
    
    mstop_i2c_wait_and_clear(peripherals)?;
    
    // disable interrupts sources
    peripherals
        .I2C0_S
        .ien
        .reset();
    
    Ok(())
}


/// Blocking read function
/// TODO: replace with advance in blocking loop
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
        delay(10000);
    }

    // clear pending commands and tx
    peripherals
        .I2C0_S
        .cmd
        .write(|w_reg| w_reg.clearpc().set_bit().cleartx().set_bit());
    delay(10000);

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
        delay(10000);
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
    delay(10000);
    peripherals
        .I2C0_S
        .cmd
        .write(|w_reg| w_reg.start().set_bit());
    delay(10000);

    acknowledge_i2c_tx(peripherals)?;
    
    // transfer write data, single byte
    peripherals
        .I2C0_S
        .txdata
        .write(|w_reg| w_reg.txdata().variant(position));
    delay(10000);

    acknowledge_i2c_tx(peripherals)?;

    // send address `(0x38 << 1)|1`, for reading data
    peripherals
        .I2C0_S
        .cmd
        .write(|w_reg| w_reg.start().set_bit());
    delay(10000);
    peripherals
        .I2C0_S
        .txdata
        .write(|w_reg| w_reg.txdata().variant(0b1110001));
    delay(10000);

    acknowledge_i2c_tx(peripherals)?;
    
    let mut rx_data_collected: Vec<u8> = Vec::with_capacity(LEN);
    
    for i in 0..LEN {
        rx_data_collected.push(read_i2c_rx(peripherals)?);
        if i == LEN-1 {
            peripherals
                .I2C0_S
                .cmd
                .write(|w_reg| w_reg.nack().set_bit());
            delay(10000);
            peripherals
                .I2C0_S
                .cmd
                .write(|w_reg| w_reg.stop().set_bit());
            delay(10000);
        } else {
            peripherals
                .I2C0_S
                .cmd
                .write(|w_reg| w_reg.ack().set_bit());
            delay(10000);
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
*/

/// Touchpad driver async state machine; value represents timer counter to counteract hardware line
/// weirdness - on count to 0 operation is supposed to be executed. Timer check does not capture
/// critical section, operation does.
pub struct Read<const LEN: usize, const POS: u8> {
    state: ReadState<LEN>,
    buffer: [u8; LEN],
    timer: usize,
}

pub enum ReadState<const LEN: usize> {
    /// Initial idle state
    Init,
    /// Safe initial state - aborting possible previous operations
    ///
    /// command and Tx buffers should be cleaned at this point
    ClearCommand,
    /// Make sure Rx is clear and start operation by preparing to send device address
    ClearRx,
    /// Initiate write communication by sending device ID (address)
    SendId,
    /// Prepare address to write data
    PrepareAddress,
    /// Send address to write data
    SendAddress,
    /// Transmit read address to device
    PrepareRead,
    /// Initiate read communication
    Read(ReadLoop<LEN>),
    /// Final state, to cleanup and report result
    Aftermath,
}

impl <const LEN: usize, const POS: u8> Read<LEN, POS> {
    fn count(&mut self) -> bool {
        if self.timer == 0 {
            false
        } else {
            self.timer -= 1;
            true
        }
    }
}

impl <const LEN: usize, const POS: u8> Operation for Read<LEN, POS> {
    type Input<'a> = ();
    type Output = Result<Option<[u8; LEN]>, I2CError>;
    type StateEnum = ReadState<LEN>;

    fn new() -> Self {
        Self {
            state: ReadState::Init,
            buffer: [0; LEN],
            timer: 0,
        }
    }

    fn wind(&mut self, state: ReadState<LEN>, delay: usize) {
        self.state = state;
        self.timer = delay;
    }

    fn advance(&mut self, _: ()) -> Result<Option<[u8; LEN]>, I2CError> {
        if self.count() { return Ok(None) };
        match self.state {
            ReadState::Init => {
                // abort unexpected processes
                in_free(|peripherals|
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
                    self.wind_d(ReadState::ClearCommand);
                } else { self.change(ReadState::ClearCommand); }
                );
                Ok(None)
            },
            ReadState::ClearCommand => {
                in_free(|peripherals|
                peripherals
                    .I2C0_S
                    .cmd
                    .write(|w_reg| w_reg.clearpc().set_bit().cleartx().set_bit())
                    );
                self.wind_d(ReadState::ClearRx);
                Ok(None)
            },
            ReadState::ClearRx => {
                if if_in_free(|peripherals|
                    peripherals
                        .I2C0_S
                        .status
                        .read()
                        .rxdatav()
                        .bit_is_set()
                )? {
                    in_free(|peripherals| {
                        let _dummy_data = peripherals
                            .I2C0_S
                            .rxdata
                            .read()
                            .bits();
                    });
                    self.wind_d(ReadState::ClearRx);
                } else {
                    in_free(|peripherals| {
                        // clear interrupt flags
                        peripherals
                            .I2C0_S
                            .if_
                            .reset();
    
                        // enable interrupts sources
                        peripherals
                            .I2C0_S
                            .ien
                            .write(|w_reg| 
                                w_reg
                                    .nack().set_bit()
                                    .ack().set_bit()
                                    .mstop().set_bit()
                                    .rxdatav().set_bit()
                                    .arblost().set_bit()
                                    .buserr().set_bit()
                            );
                    });

                    // i2c transfer sequence

                    check_i2c_errors()?;
                    // send address `0x38 << 1`, for writing data
                    in_free(|peripherals|
                        peripherals
                            .I2C0_S
                            .txdata
                            .write(|w_reg| w_reg.txdata().variant(0b1110000))
                    );
                    self.wind_d(ReadState::SendId);
                };
                Ok(None)
            },
            ReadState::SendId => {
                in_free(|peripherals|
                peripherals
                    .I2C0_S
                    .cmd
                    .write(|w_reg| w_reg.start().set_bit())
                );
                self.wind_d(ReadState::PrepareAddress);
                Ok(None)
            },
            ReadState::PrepareAddress => { //TODO expand this
                acknowledge_i2c_tx()?;
                in_free(|peripherals| 
                peripherals
                    .I2C0_S
                    .txdata
                    .write(|w_reg| w_reg.txdata().variant(POS))
                );
                self.wind_d(ReadState::SendAddress);
                Ok(None)
            },
            ReadState::SendAddress => {
                acknowledge_i2c_tx()?;
                in_free(|peripherals|
                    peripherals
                        .I2C0_S
                        .cmd
                        .write(|w_reg| w_reg.start().set_bit())
                    );
                self.wind_d(ReadState::PrepareRead);
                Ok(None)
            },
            ReadState::PrepareRead => {
                in_free(|peripherals|
                peripherals
                    .I2C0_S
                    .txdata
                    .write(|w_reg| w_reg.txdata().variant(0b1110001))
                    );
                self.change(ReadState::Read(ReadLoop::<LEN>::new()));
                Ok(None)
            },
            ReadState::Read(ref mut a) => {
                if let Some (b) = a.advance(())? {
                    self.buffer = b;
                    self.wind_d(ReadState::Aftermath);
                };
                Ok(None)
            },
            ReadState::Aftermath => {
                mstop_i2c_wait_and_clear()?;
                in_free(|peripherals|
                    peripherals
                        .I2C0_S
                        .ien
                        .reset()
                );
                self.change(ReadState::Init);
                Ok(Some(self.buffer))
            },
        }
    }
}

pub struct ReadLoop<const LEN: usize> {
    position: usize,
    value: [u8; LEN],
    state: ReadLoopState,
    timer: usize,
}

pub enum ReadLoopState {
    /// Wait and ack
    AckRead,
    /// Read cycle
    Read(ReadI2C),
    /// Stop reading and report result
    Aftermath,
}

impl <const LEN: usize> ReadLoop<LEN> {
    fn count(&mut self) -> bool {
        if self.timer == 0 {
            false
        } else {
            self.timer -= 1;
            true
        }
    }
}

impl <const LEN: usize> Operation for ReadLoop<LEN> {
    type Input<'a> = ();
    type Output = Result<Option<[u8; LEN]>, I2CError>;
    type StateEnum = ReadLoopState;

    fn new() -> Self {
        Self {
            position: 0,
            value: [0; LEN],
            state: ReadLoopState::AckRead,
            timer: DELAY,
        }
    }

    fn wind(&mut self, state: ReadLoopState, delay: usize) {
        self.state = state;
        self.timer = delay;
    }


    fn advance(&mut self, _: ()) -> Result<Option<[u8; LEN]>, I2CError> {
        if self.count() { return Ok(None) };
        match self.state {
            ReadLoopState::AckRead => {
                acknowledge_i2c_tx()?;
                self.change(ReadLoopState::Read(ReadI2C::new()));
                Ok(None)
            },
            ReadLoopState::Read(ref mut a) => {
                if let Some(b) = a.advance(())? {
                    self.value[self.position] = b;
                    if self.position == LEN-1 {
                        in_free(|peripherals| 
                            peripherals
                                .I2C0_S
                                .cmd
                                .write(|w_reg| w_reg.nack().set_bit())
                        );
                        self.wind_d(ReadLoopState::Aftermath);
                    } else {
                        in_free(|peripherals|
                            peripherals
                                .I2C0_S
                                .cmd
                                .write(|w_reg| w_reg.ack().set_bit())
                        );
                        self.wind_d(ReadLoopState::Read(ReadI2C::new()));
                        self.position += 1;
                    }
                }
                Ok(None)
            },
            ReadLoopState::Aftermath => {
                in_free(|peripherals|
                    peripherals
                        .I2C0_S
                        .cmd
                        .write(|w_reg| w_reg.stop().set_bit())
                );
                Ok(Some(self.value))
            }
        }
    }
}
