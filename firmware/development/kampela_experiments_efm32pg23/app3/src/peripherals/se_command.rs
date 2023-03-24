//! Interacting with security element through FIFO mailbox.

use cortex_m::asm::delay;

use efm32pg23_fix::Peripherals;

/// Value to use in place of a pointer when there is no input or no output to
/// pass into the command.
pub const SE_DATATRANSFER_NO_DATA: u32 = 0;

/// Value to use in place of a pointer when there is no next input or output to
/// pass.
pub const SE_DATATRANSFER_STOP: u32 = 0x00000001;

/// Data length value modifier.
pub const SE_DATATRANSFER_REALIGN: u32 = 0x20000000;

/// Create key command word.
pub const SE_COMMAND_CREATE_KEY: u32 = 0x02000000;

/// AES GCM encrypt command word.
pub const SE_COMMAND_AES_GCM_ENCRYPT: u32 = 0x04020000;

/// AES GCM decrypt command word base.
pub const SE_COMMAND_AES_GCM_DECRYPT: u32 = 0x04030000;

/// AES GSM decrypt command word for provided tag.
pub fn se_command_aes_gsm_decrypt(tag: &[u8]) -> u32 {
    SE_COMMAND_AES_GCM_DECRYPT | ((tag.len() as u32 & 0xFF) << 8)
}

/// Get random command word.
pub const SE_COMMAND_TRNG_GET_RANDOM: u32 = 0x07000000;

/// Command for SE mailbox.
///
/// Contains pointers. All input and output elements must remain in scope when command goes into FIFO.
pub struct SeCommand<'a> {
    /// Command word, values taken from SDK
    pub command_word: u32,

    /// `u32` representation of pointer to the first [`DataTransfer`] of the input.
    ///
    /// If there is no input, [`SE_DATATRANSFER_NO_DATA`] is used instead.
    pub data_in: u32,

    /// `u32` representation of pointer to the first [`DataTransfer`] of the output.
    ///
    /// If there is no output, [`SE_DATATRANSFER_NO_DATA`] is used instead.
    pub data_out: u32,

    /// Command parameters, 4 or fewer.
    pub parameters: &'a [u32],
}

/// Data transfer element.
#[derive(Debug)]
#[repr(C)]
pub struct DataTransfer {
    /// `u32` representation of pointer to current `&[u8]` slice.
    pub data: u32,

    /// `u32` representation of pointer to next [`DataTransfer`].
    ///
    /// If there are no further ones, [`SE_DATATRANSFER_STOP`] is used.
    pub next: u32,

    /// Length of the current `&[u8]` slice.
    pub length: u32,
}

/// Decrement of remaining bytes expected in FIFO.
///
/// Data goes into FIFO as u32. This is just u32 size in bytes.
pub const REMBYTES_DECREMENT: u16 = 4;

impl<'a> SeCommand<'a> {
    /// Execute command in SE.
    ///
    /// For this to work, SE must be already enabled elsewhere.
    ///
    /// Sends header and then all data pieces in FIFO, with appropriate wait
    /// periods to allow for data processing.
    pub fn execute(&self, peripherals: &mut Peripherals) -> Result<(), RxError> {
        // TXINT bit is set when there is space in FIFO for at least 16 words.
        // Wait for it here.
        while peripherals
            .SEMAILBOX_S_HOST
            .tx_status
            .read()
            .txint()
            .bit_is_clear()
        {}

        // Header.
        // Provides the initial number of expected remaining bytes.
        // After each new register writing must wait for appropriate number of
        // remaining bytes to be read from TX_STATUS, before loading new
        // value.
        let into_header = (core::mem::size_of::<u32>() * (4usize + self.parameters.len())) as u16;
        let mut rembytes = into_header;

        // Header is sent into TX_HEADER register.
        peripherals
            .SEMAILBOX_S_HOST
            .tx_header
            .write(|w_reg| w_reg.txheader().variant(into_header as u32));
        rembytes -= REMBYTES_DECREMENT;
        expect_rembytes(peripherals, rembytes);

        // Command word is sent into FIFO register.
        peripherals
            .SEMAILBOX_S_HOST
            .fifo
            .write(|w_reg| w_reg.fifo().variant(self.command_word));
        rembytes -= REMBYTES_DECREMENT;
        expect_rembytes(peripherals, rembytes);

        // Pointer to the first [`DataTransfer`] of the input is sent into FIFO
        // register.
        // SE gets further input elements through pointers in [`DataTransfer`].
        // Input ends when `SE_DATATRANSFER_STOP` is encoutered.
        peripherals
            .SEMAILBOX_S_HOST
            .fifo
            .write(|w_reg| w_reg.fifo().variant(self.data_in));
        rembytes -= REMBYTES_DECREMENT;
        expect_rembytes(peripherals, rembytes);

        // Pointer to the first [`DataTransfer`] of the output is sent into FIFO
        // register.
        // SE gets further input elements through pointers in [`DataTransfer`].
        // Output ends when `SE_DATATRANSFER_STOP` is encoutered.
        peripherals
            .SEMAILBOX_S_HOST
            .fifo
            .write(|w_reg| w_reg.fifo().variant(self.data_out));
        rembytes -= REMBYTES_DECREMENT;
        expect_rembytes(peripherals, rembytes);

        // Parameters are sent into FIFO register.
        for param in self.parameters.iter() {
            peripherals
                .SEMAILBOX_S_HOST
                .fifo
                .write(|w_reg| w_reg.fifo().variant(*param));
            rembytes -= REMBYTES_DECREMENT;
            expect_rembytes(peripherals, rembytes);
        }

        // RXINT is set when there are at least 4 words in the FIFO or if the
        // final word of the message is present in the FIFO.
        // Wait for it here.
        while peripherals
            .SEMAILBOX_S_HOST
            .rx_status
            .read()
            .rxint()
            .bit_is_clear()
        {}

        // All data is processed when RX_HEADER gets empty.
        loop {
            // This does not work properly without the delay.
            // Minimal delay is sufficient.
            delay(1);
            let rx_header = peripherals.SEMAILBOX_S_HOST.rx_header.read().bits();
            if rx_header.eq(&0) {
                break;
            }
        }

        match peripherals
            .SEMAILBOX_S_HOST
            .rx_status
            .read()
            .msginfo()
            .bits()
        {
            0 => Ok(()),
            1 => Err(RxError::InvalidCommand),
            2 => Err(RxError::AuthorizationError),
            3 => Err(RxError::InvalidSignature),
            4 => Err(RxError::BusError),
            5 => Err(RxError::InternalError),
            6 => Err(RxError::CryptoError),
            7 => Err(RxError::InvalidParameter),
            9 => Err(RxError::Abort),
            _ => Err(RxError::Unspecified),
        }
    }
}

/// Errors from RX_STATUS response.
///
/// Some hints regarding error meaning are provided in documentation for
/// related, but different chip efr32mg21:
/// <https://docs.silabs.com/mcu/latest/efr32mg21/group-SE>.
#[derive(Debug, Eq, PartialEq)]
pub enum RxError {
    Abort,
    AuthorizationError,
    BusError,
    CryptoError,
    InternalError,
    InvalidCommand,
    InvalidParameter,
    InvalidSignature,
    Unspecified,
}

/// Wait for expected remaining bytes value.
fn expect_rembytes(peripherals: &mut Peripherals, rembytes: u16) {
    while peripherals
        .SEMAILBOX_S_HOST
        .tx_status
        .read()
        .rembytes()
        .bits()
        .ne(&rembytes)
    {}
}
