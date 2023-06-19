//! Using random number generator in security element.

use alloc::{vec, vec::Vec};
use core::{mem::size_of, ptr::addr_of};

use rand_core::{CryptoRng, Error, RngCore};

use crate::peripherals::se_command::{
    DataTransfer, RxError, SeCommand, SE_COMMAND_TRNG_GET_RANDOM,
    SE_DATATRANSFER_NO_DATA, SE_DATATRANSFER_REALIGN, SE_DATATRANSFER_STOP,
};

use crate::in_free;

pub struct SeRng {}

impl RngCore for SeRng {
    fn next_u32(&mut self) -> u32 {
        let bytes = random_with_length(size_of::<u32>())
            .expect("expected rng normal functioning");
        u32::from_be_bytes(bytes.try_into().expect("stable u32 length"))
    }
    fn next_u64(&mut self) -> u64 {
        let bytes = random_with_length(size_of::<u64>())
            .expect("expected rng normal functioning");
        u64::from_be_bytes(bytes.try_into().expect("stable u64 length"))
    }
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        let content = random_with_length(dest.len())
            .expect("expected rng normal functioning");
        dest.copy_from_slice(&content[..dest.len()]);
    }
    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.fill_bytes(dest);
        Ok(())
    }
}

impl CryptoRng for SeRng {}

pub const PAGE_SIZE: usize = 4;

pub fn random_with_length(len: usize) -> Result<Vec<u8>, RxError> {
    // bytes in whole 4-byte pages
    let even = len & !0x3;

    // remaining bytes in last 4-byte page
    let odd = len & 0x3;

    // `len` capped to nearest full 4-byte page set must be below heap capacity
    //
    // Note: allocator may panic here.
    //
    // Values must be pre-set, FIFO can not output if only `with_capacity(_)` is
    // used.
    let mut out: Vec<u8> = vec![0; even + PAGE_SIZE];

    let command_word = SE_COMMAND_TRNG_GET_RANDOM;
    let data_in = SE_DATATRANSFER_NO_DATA;

    if even > 0 {
        let data_transfer_out0 = DataTransfer {
            data: addr_of!(out[0]) as u32,
            next: SE_DATATRANSFER_STOP,
            length: even as u32 | SE_DATATRANSFER_REALIGN,
        };

        let data_out = addr_of!(data_transfer_out0) as u32;

        let parameters = [even as u32];

        let se_command = SeCommand {
            command_word,
            data_in,
            data_out,
            parameters: parameters.as_slice(),
        };
        
        in_free(|peripherals| {
            se_command.execute(peripherals).expect("se_rng communication error");
        }
        );
    }

    if odd > 0 {
        let data_transfer_out0 = DataTransfer {
            data: addr_of!(out[even]) as u32,
            next: SE_DATATRANSFER_STOP,
            length: PAGE_SIZE as u32 | SE_DATATRANSFER_REALIGN,
        };

        let data_out = addr_of!(data_transfer_out0) as u32;

        let parameters = [PAGE_SIZE as u32];

        let se_command = SeCommand {
            command_word,
            data_in,
            data_out,
            parameters: parameters.as_slice(),
        };

        in_free(|peripherals| {
            se_command.execute(peripherals).expect("se_rng communication error");
        }
        );
    }

    out.truncate(len);

    Ok(out)
}
