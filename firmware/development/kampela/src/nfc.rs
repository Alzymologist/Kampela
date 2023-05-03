//! NFC packet collector and decoder


use nfca_parser::{frame::{Frame, FrameAttributed}, miller::*, time_record_both_ways::*};
use efm32pg23_fix::{CorePeripherals, interrupt, Interrupt, NVIC, Peripherals};
use alloc::{borrow::ToOwned, vec::Vec};

use kampela_system::{
    PERIPHERALS, CORE_PERIPHERALS, in_free,
    devices::{power::measure_voltage, se_rng, touch::{FT6X36_REG_NUM_TOUCHES, LEN_NUM_TOUCHES}},
    init::init_peripherals,
    BUF_QUARTER, LINK_1, LINK_2, LINK_DESCRIPTORS, TIMER0_CC0_ICF, NfcXfer, NfcXferBlock,
};
use cortex_m::interrupt::free;
use crate::BUFFER_INFO;

pub const FREQ: u16 = 22;

pub struct BufferInfo {
    pub buffer_status: BufferStatus,
    pub maybe_previous_tail: Option<MillerTimesDown<FREQ>>,
}

impl BufferInfo {
    pub fn new() -> Self {
        Self {
            buffer_status: BufferStatus::new(),
            maybe_previous_tail: None,
        }
    }
}

pub enum BufferStatus {
    R0W1,
    R0W2,
    R0Wh,
    R1W2,
    R1W3,
    R1Wh,
    R2W0,
    R2W3,
    R2Wh,
    R3W0,
    R3W1,
    R3Wh,
    RhW0,
    RhW1,
    RhW2,
    RhW3,
}

pub enum BufRegion {
    Reg0,
    Reg1,
    Reg2,
    Reg3,
}


#[derive(Debug)]
pub enum BufferError {
    UnexpectedIfDone7,
    UnexpectedReadDone,
}

impl BufferStatus {
    pub fn new() -> Self {
        Self::RhW0
    }
    pub fn pass_if_done7(&mut self) -> Result<(), BufferError> {
        let new_self = match self {
            Self::R0W1 => Self::R0W2,
            Self::R0W2 => Self::R0Wh,
            Self::R0Wh => return Err(BufferError::UnexpectedIfDone7),
            Self::R1W2 => Self::R1W3,
            Self::R1W3 => Self::R1Wh,
            Self::R1Wh => return Err(BufferError::UnexpectedIfDone7),
            Self::R2W0 => Self::R2Wh,
            Self::R2W3 => Self::R2W0,
            Self::R2Wh => return Err(BufferError::UnexpectedIfDone7),
            Self::R3W0 => Self::R3W1,
            Self::R3W1 => Self::R3Wh,
            Self::R3Wh => return Err(BufferError::UnexpectedIfDone7),
            Self::RhW0 => Self::R0W1,
            Self::RhW1 => Self::R1W2,
            Self::RhW2 => Self::R2W3,
            Self::RhW3 => Self::R3W0,
        };
        *self = new_self;
        Ok(())
    }
    pub fn pass_read_done(&mut self) -> Result<(), BufferError> {
        let new_self = match self {
            Self::R0W1 => Self::RhW1,
            Self::R0W2 => Self::R1W2,
            Self::R0Wh => Self::R1W3,
            Self::R1W2 => Self::RhW2,
            Self::R1W3 => Self::R2W3,
            Self::R1Wh => Self::R2W0,
            Self::R2W0 => Self::R3W0,
            Self::R2W3 => Self::RhW3,
            Self::R2Wh => Self::R3W1,
            Self::R3W0 => Self::RhW0,
            Self::R3W1 => Self::R0W1,
            Self::R3Wh => Self::R0W2,
            Self::RhW0 => return Err(BufferError::UnexpectedReadDone),
            Self::RhW1 => return Err(BufferError::UnexpectedReadDone),
            Self::RhW2 => return Err(BufferError::UnexpectedReadDone),
            Self::RhW3 => return Err(BufferError::UnexpectedReadDone),
        };
        *self = new_self;
        Ok(())
    }
    pub fn read_from(&self) -> Option<BufRegion> {
        match self {
            Self::R0W1 => Some(BufRegion::Reg0),
            Self::R0W2 => Some(BufRegion::Reg0),
            Self::R0Wh => Some(BufRegion::Reg0),
            Self::R1W2 => Some(BufRegion::Reg1),
            Self::R1W3 => Some(BufRegion::Reg1),
            Self::R1Wh => Some(BufRegion::Reg1),
            Self::R2W0 => Some(BufRegion::Reg2),
            Self::R2W3 => Some(BufRegion::Reg2),
            Self::R2Wh => Some(BufRegion::Reg2),
            Self::R3W0 => Some(BufRegion::Reg3),
            Self::R3W1 => Some(BufRegion::Reg3),
            Self::R3Wh => Some(BufRegion::Reg3),
            Self::RhW0 => None,
            Self::RhW1 => None,
            Self::RhW2 => None,
            Self::RhW3 => None,
        }
    }
    pub fn is_write_halted(&self) -> bool {
        match self {
            Self::R0W1 => false,
            Self::R0W2 => false,
            Self::R0Wh => true,
            Self::R1W2 => false,
            Self::R1W3 => false,
            Self::R1Wh => true,
            Self::R2W0 => false,
            Self::R2W3 => false,
            Self::R2Wh => true,
            Self::R3W0 => false,
            Self::R3W1 => false,
            Self::R3Wh => true,
            Self::RhW0 => false,
            Self::RhW1 => false,
            Self::RhW2 => false,
            Self::RhW3 => false,
        }
    }
}

const TARGET_PACKET_LEN: usize = 252;

pub struct MillerProcessed {
    frame_set: Vec<Frame>,
    tail: MillerTimesDown<FREQ>,
}

pub fn get_miller_frames(input: &[u16], maybe_previous_tail: Option<MillerTimesDown<FREQ>>) -> MillerProcessed {
    let times_down = MillerTimesDown::<FREQ>::from_raw(input);
    let (new_nose, full_frames_with_tail) = times_down.split_first().expect("expect at least one break with current buffer settings");
    let first_chunk = {
        if let Some(previous_tail) = maybe_previous_tail {
            new_nose.stitch_with_tail(&previous_tail)
        }
        else {new_nose.to_owned()}
    };
    let (new_tail, full_frames) = full_frames_with_tail.split_last().map(|(tail, full_frames)| (tail.to_owned(), full_frames)).expect("expect at least one break with current buffer settings");
    let mut frame_set: Vec<Frame> = Vec::new();
    for chunk in core::iter::once(&first_chunk).chain(full_frames.into_iter()) {
        if let Ok(miller_element_set) = chunk.convert() {
            if let Ok(frame) = miller_element_set.collect_frame() {
                if let Frame::Standard(ref standard_frame) = frame {
                    if standard_frame.len() == TARGET_PACKET_LEN {
                        frame_set.push(frame);
                    }
                }
            }
        }
    }
    MillerProcessed {
        frame_set,
        tail: new_tail,
    }
}

pub fn process_nfc_buffer_miller_only(frame_set: &mut Vec<Frame>, nfc_buffer: &[u16; 4*BUF_QUARTER]) {
    let mut read_from = None;
    let mut maybe_previous_tail = None;
    free(|cs| {
        let buffer_info = BUFFER_INFO.borrow(cs).borrow();
        read_from = buffer_info.buffer_status.read_from();
        maybe_previous_tail = buffer_info.maybe_previous_tail.clone();
    });
    let decoder_input = match read_from {
        Some(BufRegion::Reg0) => &nfc_buffer[..BUF_QUARTER],
        Some(BufRegion::Reg1) => &nfc_buffer[BUF_QUARTER..2*BUF_QUARTER],
        Some(BufRegion::Reg2) => &nfc_buffer[2*BUF_QUARTER..3*BUF_QUARTER],
        Some(BufRegion::Reg3) => &nfc_buffer[3*BUF_QUARTER..],
        None => return,
    };
    let mut miller_processed = get_miller_frames(decoder_input, maybe_previous_tail);
    frame_set.append(&mut miller_processed.frame_set);
    free(|cs| {
        let mut buffer_info = BUFFER_INFO.borrow(cs).borrow_mut();
        buffer_info.maybe_previous_tail = Some(miller_processed.tail);
        let was_write_halted = buffer_info.buffer_status.is_write_halted();
        buffer_info.buffer_status.pass_read_done().expect("to do");
        if was_write_halted {
            panic!("was write halted");
            unsafe {
                NVIC::unmask(Interrupt::LDMA);
            }
        }
    });
}

