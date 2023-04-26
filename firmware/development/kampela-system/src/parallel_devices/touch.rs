//! Async touch panel operations
//!
//! This matches devices::touch blocking flow; TODO: replace that flow with this one

use efm32pg23_fix::Peripherals;
use crate::peripherals::i2c::{acknowledge_i2c_tx, check_i2c_errors, I2CError, read_i2c_rx};
use crate::parallel_devices::Operation;

const DELAY: usize = 10000;
const START: usize = 1;

/// Touchpad driver async state machine; value represents timer counter to counteract hardware line
/// weirdness - on count to 0 operation is supposed to be executed. Timer check does not capture
/// critical section, operation does.
pub struct Read<const LEN: usize, const POS: u8> {
    state: ReadState,
    buffer: [u8; LEN]
}

enum ReadState {
    /// Initial idle state
    Init,
    /// Safe initial state - aborting possible previous operations
    ///
    /// command and Tx buffers should be cleaned at this point
    ClearCommand(usize),
    /// Make sure Rx is clear and start operation by preparing to send device address
    ClearRx(usize),
    /// Initiate write communication by sending device ID (address)
    SendId(usize),
    /// Prepare address to write data
    PrepareAddress(usize),
    /// Transmit read address to device
    SendAddress(usize),
    /// Initiate read communication
    Read(ReadLoop),
}

fn count(counter: &mut usize) -> bool {
    *counter -= 1;
    *counter == 0
}

impl <const LEN: usize, const POS: u8> Operation for Read<LEN, POS> {
    type DesiredOutput = Result<Option<[u8; LEN]>, I2CError>;

    fn new() -> Self {
        Self {
            state: ReadState::Init,
            buffer: [0; LEN],
        }
    }

    fn advance_check(&mut self) -> bool {
        match self.state {
            ReadState::Init => true,
            ReadState::ClearCommand(ref mut a) => count(a),
            ReadState::ClearRx(ref mut a) => count(a),
            ReadState::SendId(ref mut a) => count(a),
            ReadState::PrepareAddress(ref mut a) => count(a),
            ReadState::SendAddress(ref mut a) => count(a),
            ReadState::Read(ref mut a) => a.advance_check(),
        }
    }

    
    fn advance(&mut self, peripherals: &mut Peripherals) -> Result<Option<[u8; LEN]>, I2CError> {
        match self.state {
            ReadState::Init => {
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
                    self.state = ReadState::ClearCommand(DELAY);
                } else { self.state = ReadState::ClearCommand(START); }
                Ok(None)
            },
            ReadState::ClearCommand(_) => {
                peripherals
                    .I2C0_S
                    .cmd
                    .write(|w_reg| w_reg.clearpc().set_bit().cleartx().set_bit());
                self.state = ReadState::ClearRx(DELAY);
                Ok(None)
            },
            ReadState::ClearRx(_) => {
                if peripherals
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
                    self.state = ReadState::ClearRx(DELAY);
                } else {
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
                    self.state = ReadState::SendId(DELAY);
                }
                Ok(None)
            },
            ReadState::SendId(_) => {
                peripherals
                    .I2C0_S
                    .cmd
                    .write(|w_reg| w_reg.start().set_bit());
                self.state = ReadState::PrepareAddress(DELAY);
                Ok(None)
            },
            ReadState::PrepareAddress(_) => { //TODO expand this
                acknowledge_i2c_tx(peripherals)?;
                peripherals
                    .I2C0_S
                    .txdata
                    .write(|w_reg| w_reg.txdata().variant(POS));
                self.state = ReadState::SendAddress(DELAY);
                Ok(None)
            },
            ReadState::SendAddress(_) => {
                peripherals
                    .I2C0_S
                    .txdata
                    .write(|w_reg| w_reg.txdata().variant(0b1110001));
                self.state = ReadState::Read(ReadLoop::new());
                Ok(None)
            },
            ReadState::Read(ref mut a) => {
                Ok(None)
            },
        }
    }
}

struct ReadLoop {
    delay: usize,
    position: usize,
    state: ReadLoopState,
}

enum ReadLoopState {
    AckRead,
    Read(usize),
}

impl Operation for ReadLoop {
    type DesiredOutput = Result<Option<u8>, I2CError>;

    fn new() -> Self {
        Self {
            delay: DELAY,
            position: 0,
            state: ReadLoopState::AckRead,
        }
    }

    fn advance_check(&mut self) -> bool {
        match self.state {
            ReadLoopState::AckRead => true,
            ReadLoopState::Read(ref mut a) => count(a),
        }
    }

    fn advance(&mut self, peripherals: &mut Peripherals) -> Result<Option<u8>, I2CError> {
        match self.state {
            ReadLoopState::AckRead => {
                acknowledge_i2c_tx(peripherals)?;
                self.state = ReadLoopState::Read(START);
                Ok(None)
            },
            ReadLoopState::Read(_) => {
                let out = read_i2c_rx(peripherals)?;
                Ok(None)
            },
        }
    }
}
