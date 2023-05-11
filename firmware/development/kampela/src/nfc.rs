//! NFC packet collector and decoder


use nfca_parser::{frame::{Frame, FrameAttributed}, miller::*, time_record_both_ways::*};
use efm32pg23_fix::{CorePeripherals, interrupt, Interrupt, NVIC, Peripherals};
use alloc::{borrow::ToOwned, vec::Vec};

use kampela_system::{
    PERIPHERALS, CORE_PERIPHERALS, in_free,
    devices::{se_rng, touch::{FT6X36_REG_NUM_TOUCHES, LEN_NUM_TOUCHES}},
    init::init_peripherals,
    BUF_QUARTER, LINK_1, LINK_2, LINK_DESCRIPTORS, TIMER0_CC0_ICF, NfcXfer, NfcXferBlock,
};
use cortex_m::interrupt::free;
use crate::BUFFER_INFO;

use kampela_common::{NFC_PACKET_FULL_SIZE, NfcPacket};
use kampela_system::devices::psram::{AddressPsram, ExternalPsram, PsramAccess, psram_read_at_address};
use raptorq_metal_wrap::{DataAtAddress, DecoderMetal, ExternalAddress};
use raptorq::EncodingPacket;
use substrate_parser::compacts::find_compact;

use core::ops::DerefMut;

pub const FREQ: u16 = 22;

#[derive(Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Debug)]
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

pub struct MillerProcessed {
    frame_set: Vec<[u8; 240]>,
    tail: MillerTimesDown<FREQ>,
}

pub fn get_miller_frames(input: &[u16], maybe_previous_tail: Option<MillerTimesDown<FREQ>>) -> Option<MillerProcessed> {
    let times_down = MillerTimesDown::<FREQ>::from_raw(input);
    match times_down.split_first() {
        Some((new_nose, full_frames_with_tail)) => {
            let first_chunk = {
                if let Some(previous_tail) = maybe_previous_tail {
                    new_nose.stitch_with_tail(&previous_tail)
                }
                else {new_nose.to_owned()}
            };
            match full_frames_with_tail.split_last() {
                Some((new_tail, full_frames)) => {
                    let mut frame_set: Vec<[u8;240]> = Vec::new();
                    for chunk in core::iter::once(&first_chunk).chain(full_frames.into_iter()) {
                        if let Ok(miller_element_set) = chunk.convert() {
                            if let Ok(frame) = miller_element_set.collect_frame() {
                                if let Frame::Standard(standard_frame) = frame {
                                    if standard_frame.len() == 240 {
                                        frame_set.push(standard_frame.try_into().expect("checked size"));
/*
                                        if !frame_set.contains(&frame) {
                                            frame_set.push(frame);
                                        }
*/
                                    }
                                }
                            }
                        }
                    }
                    Some(MillerProcessed {
                        frame_set,
                        tail: new_tail.to_owned(),
                    })
                },
                None => None //panic!("can not split buffer tail")
            }
        },
        None => None//panic!("can not split buffer nose"), //clean tail here TODO
    }
}

pub fn process_nfc_buffer_miller_only(frame_set: &mut Vec<[u8; 240]>, nfc_buffer: &[u16; 4*BUF_QUARTER], counter: &mut usize) {
    let mut read_from = None;
    let mut maybe_previous_tail = None;
    free(|cs| {
        let buffer_info = BUFFER_INFO.borrow(cs).borrow();
        read_from = buffer_info.buffer_status.read_from();
        maybe_previous_tail = buffer_info.maybe_previous_tail.clone();
        if *counter >= 10 {panic!("at counter {counter}: {:?}", buffer_info);}
    });
    let decoder_input = match read_from {
        Some(BufRegion::Reg0) => &nfc_buffer[..BUF_QUARTER],
        Some(BufRegion::Reg1) => &nfc_buffer[BUF_QUARTER..2*BUF_QUARTER],
        Some(BufRegion::Reg2) => &nfc_buffer[2*BUF_QUARTER..3*BUF_QUARTER],
        Some(BufRegion::Reg3) => &nfc_buffer[3*BUF_QUARTER..],
        None => return,
    };
    *counter += 1;
    if let Some(mut miller_processed) = get_miller_frames(decoder_input, maybe_previous_tail) {
        frame_set.append(&mut miller_processed.frame_set);
        free(|cs| {
            let mut buffer_info = BUFFER_INFO.borrow(cs).borrow_mut();
            buffer_info.maybe_previous_tail = Some(miller_processed.tail);
            let was_write_halted = buffer_info.buffer_status.is_write_halted();
            buffer_info.buffer_status.pass_read_done().expect("to do");
            if was_write_halted & ! buffer_info.buffer_status.is_write_halted() {
                if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
//                    peripherals.LDMA_S.if_.reset();
                    peripherals.LDMA_S.linkload.write(|w_reg| w_reg.linkload().variant(0b11111111));
                }
                else {panic!("can not borrow peripherals, counter: {counter}, buffer_info: {:?}, got some new frames", buffer_info)}
            }
        });
    }
    else {
        free(|cs| {
            let mut buffer_info = BUFFER_INFO.borrow(cs).borrow_mut();
            buffer_info.maybe_previous_tail = None;
            let was_write_halted = buffer_info.buffer_status.is_write_halted();
            buffer_info.buffer_status.pass_read_done().expect("to do");
            if was_write_halted & ! buffer_info.buffer_status.is_write_halted() {
                if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
//                    peripherals.LDMA_S.if_.reset();
                    peripherals.LDMA_S.linkload.write(|w_reg| w_reg.linkload().variant(0b11111111));
                }
                else {panic!("can not borrow peripherals, counter: {counter}, buffer_info: {:?}, NO new frames", buffer_info)}
            }
        });
    }
}

pub fn turn_nfc_collector(collector: &mut NfcCollector, nfc_buffer: &[u16; 4*BUF_QUARTER]) {
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
    if let Some(miller_processed) = get_miller_frames(decoder_input, maybe_previous_tail) {
        for frame in miller_processed.frame_set.into_iter() {
            let nfc_packet = NfcPacket::from_raw(frame);
            in_free(|peripherals| {
                let mut external_psram = ExternalPsram{peripherals};
                collector.add_packet(&mut external_psram, &nfc_packet);
            });
        }
        free(|cs| {
            let mut buffer_info = BUFFER_INFO.borrow(cs).borrow_mut();
            buffer_info.maybe_previous_tail = Some(miller_processed.tail);
            let was_write_halted = buffer_info.buffer_status.is_write_halted();
            buffer_info.buffer_status.pass_read_done().expect("to do");
            if was_write_halted & ! buffer_info.buffer_status.is_write_halted() {
                if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
                    peripherals.LDMA_S.linkload.write(|w_reg| w_reg.linkload().variant(0b11111111));
                }
                else {panic!("can not borrow peripherals, buffer_info: {:?}, got some new frames", buffer_info)}
            }
        });
    }
    else {
        free(|cs| {
            let mut buffer_info = BUFFER_INFO.borrow(cs).borrow_mut();
            buffer_info.maybe_previous_tail = None;
            let was_write_halted = buffer_info.buffer_status.is_write_halted();
            buffer_info.buffer_status.pass_read_done().expect("to do");
            if was_write_halted & ! buffer_info.buffer_status.is_write_halted() {
                if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
                    peripherals.LDMA_S.linkload.write(|w_reg| w_reg.linkload().variant(0b11111111));
                }
                else {panic!("can not borrow peripherals, buffer_info: {:?}, NO new frames", buffer_info)}
            }
        });
    }
}

pub enum NfcCollector {
    Empty,
    InProgress{payload_length: usize, decoder_metal: DecoderMetal<AddressPsram>},
    Done(DataAtAddress<AddressPsram>)
}

impl NfcCollector {
    pub fn new() -> Self {
        Self::Empty
    }
    pub fn add_packet(&mut self, external_psram: &mut ExternalPsram, nfc_packet: &NfcPacket) {
        match self {
            NfcCollector::Empty => {
                let payload_length = nfc_packet.payload_length();
                let decoder_metal = DecoderMetal::new::<ExternalPsram>(payload_length, AddressPsram::zero());
                match decoder_metal.get_result() {
                    None => *self = NfcCollector::InProgress{payload_length, decoder_metal},
                    Some(a) => *self = NfcCollector::Done(a),
                }
            },
            NfcCollector::InProgress{payload_length, decoder_metal} => {
                let new_packet_payload_length = nfc_packet.payload_length();
                let encoding_packet = EncodingPacket::deserialize(&nfc_packet.data());
                if new_packet_payload_length == *payload_length {
                    decoder_metal.add_new_packet(external_psram, encoding_packet);
                }
                if let Some(a) = decoder_metal.get_result() {
                    *self = NfcCollector::Done(a);
                }
            },
            NfcCollector::Done(_) => {},
        }
    }
}

#[derive(Debug)]
pub enum NfcPayloadError {
    AccessOnPayload,
    AccessOnPublicKey,
    AccessOnSignature,
    ExcessData,
    NoCompactPayload,
    NoCompactPublicKey,
    NoCompactSignature,
}

#[derive(Debug)]
pub struct TransferDataReceived {
    pub encoded_data: PsramAccess,
    pub companion_signature: Vec<u8>,
    pub companion_public_key: Vec<u8>,
}

pub fn process_nfc_payload(completed_collector: DataAtAddress<AddressPsram>) -> Result<TransferDataReceived, NfcPayloadError> {
    let psram_data = PsramAccess {
        start_address: completed_collector.start_address,
        total_len: completed_collector.total_len,
    };

    let mut position = 0usize; // *relative* position in PsramAccess!

    let mut try_encoded_data = None;
    in_free(|peripherals| {
        let mut external_psram = ExternalPsram{peripherals};
        let found_compact = find_compact::<u32, PsramAccess, ExternalPsram>(&psram_data, &mut external_psram, position).unwrap(); //map_err(|_| NfcPayloadError::NoCompactPayload)?;
        try_encoded_data = Some(PsramAccess {
            start_address: AddressPsram::new(found_compact.start_next_unit as u32).unwrap(),
            total_len: found_compact.compact as usize,
        });
        position = found_compact.start_next_unit + found_compact.compact as usize;
    });
    let encoded_data = match try_encoded_data {
        Some(a) => a,
        None => return Err(NfcPayloadError::AccessOnPayload),
    };

    let mut try_companion_signature = None;
    in_free(|peripherals| {
        let mut external_psram = ExternalPsram{peripherals};
        let found_compact = find_compact::<u32, PsramAccess, ExternalPsram>(&psram_data, &mut external_psram, position).unwrap(); //.map_err(|_| NfcPayloadError::NoCompactSignature)?;
        let signature_data = psram_read_at_address(external_psram.peripherals, AddressPsram::new(found_compact.start_next_unit as u32).unwrap(), found_compact.compact as usize).unwrap(); //.map_err(|_| NfcPayloadError::AccessOnSignature)?;
        try_companion_signature = Some(signature_data);
        position = found_compact.start_next_unit + found_compact.compact as usize;
    });
    let companion_signature = match try_companion_signature {
        Some(a) => a,
        None => return Err(NfcPayloadError::AccessOnSignature),
    };

    let mut try_companion_public_key = None;
    in_free(|peripherals| {
        let mut external_psram = ExternalPsram{peripherals};
        let found_compact = find_compact::<u32, PsramAccess, ExternalPsram>(&psram_data, &mut external_psram, position).unwrap(); //.map_err(|_| NfcPayloadError::NoCompactSignature)?;
        let public_key_data = psram_read_at_address(external_psram.peripherals, AddressPsram::new(found_compact.start_next_unit as u32).unwrap(), found_compact.compact as usize).unwrap(); //.map_err(|_| NfcPayloadError::AccessOnSignature)?;
        try_companion_public_key = Some(public_key_data);
        position = found_compact.start_next_unit + found_compact.compact as usize;
    });
    let companion_public_key = match try_companion_public_key {
        Some(a) => a,
        None => return Err(NfcPayloadError::AccessOnPublicKey),
    };

    if position != psram_data.total_len {Err(NfcPayloadError::ExcessData)}
    else {Ok(TransferDataReceived{
        encoded_data,
        companion_signature,
        companion_public_key
    })}
}
