//! NFC packet collector and decoder

use nfca_parser::frame::Frame;
use alloc::vec::Vec;

use kampela_system::{
    PERIPHERALS, in_free, BUF_QUARTER, CH_TIM0,
};
use cortex_m::interrupt::free;
use crate::{BUFFER_INFO, COUNTER, UNPROCESSED_FRAMES};

use kampela_system::devices::psram::{AddressPsram, ExternalPsram, PsramAccess, psram_read_at_address};
use lt_codes::{decoder_metal::ExternalData, mock_worst_case::DecoderMetal, packet::{Packet, PACKET_SIZE}};
use substrate_parser::compacts::find_compact;

use core::ops::DerefMut;

pub const FREQ: u16 = 22;

#[derive(Debug)]
pub struct BufferInfo {
    pub buffer_status: BufferStatus,
}

impl BufferInfo {
    pub fn new() -> Self {
        Self {
            buffer_status: BufferStatus::new(),
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

pub static mut GOT_FRAMES: usize = 0;
pub static mut PARTICIPATED_PACKETS: Vec<u16> = Vec::new();
pub static mut IN_BUFFER: u16 = 0;

pub fn turn_nfc_collector_correctly(collector: &mut NfcCollector, nfc_buffer: &[u16; 4*BUF_QUARTER]) {
    let mut read_from = None;
    free(|cs| {
        let buffer_info = BUFFER_INFO.borrow(cs).borrow();
        read_from = buffer_info.buffer_status.read_from();
//        maybe_previous_tail = buffer_info.maybe_previous_tail.clone();
    });
    let decoder_input = match read_from {
        Some(BufRegion::Reg0) => &nfc_buffer[..BUF_QUARTER],
        Some(BufRegion::Reg1) => &nfc_buffer[BUF_QUARTER..2*BUF_QUARTER],
        Some(BufRegion::Reg2) => &nfc_buffer[2*BUF_QUARTER..3*BUF_QUARTER],
        Some(BufRegion::Reg3) => &nfc_buffer[3*BUF_QUARTER..],
        None => return,
    };
    unsafe {COUNTER += 1};
    let frames = Frame::process_buffer_miller_skip_tails::<_, FREQ>(decoder_input, |frame| frame_selected(&frame));
    unsafe {UNPROCESSED_FRAMES += frames.len()};

    for frame in frames.into_iter() {
        if let Frame::Standard(standard_frame) = frame {
            let serialized_packet = standard_frame[standard_frame.len() - PACKET_SIZE..].try_into().expect("static length, always fits");
            unsafe {GOT_FRAMES += 1}
            in_free(|peripherals| {
                let mut external_psram = ExternalPsram{peripherals};
                let packet = Packet::deserialize(serialized_packet);
                unsafe {PARTICIPATED_PACKETS.push(packet.id);}
                collector.add_packet(&mut external_psram, packet);
            });
        }
        else {unreachable!()}
    }
//    if let NfcCollector::InProgress(metal_decoder) = collector {unsafe {
//        IN_BUFFER = metal_decoder.number_packets_in_buffer;
//    }}

    free(|cs| {
        let mut buffer_info = BUFFER_INFO.borrow(cs).borrow_mut();
//        buffer_info.maybe_previous_tail = new_previous_tail;
        let was_write_halted = buffer_info.buffer_status.is_write_halted();
        buffer_info.buffer_status.pass_read_done().expect("to do");
        if was_write_halted & ! buffer_info.buffer_status.is_write_halted() {
            if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().deref_mut() {
                peripherals.LDMA_S.linkload.write(|w_reg| w_reg.linkload().variant(1 << CH_TIM0));
            }
            else {panic!("can not borrow peripherals, buffer_info: {:?}, got some new frames", buffer_info)}
        }
    });
}

fn frame_selected(frame: &Frame) -> bool {
    if let Frame::Standard(standard_frame) = frame {
        if standard_frame.len() >= PACKET_SIZE {true}
        else {false}
    }
    else {false}
}
/*
fn process_element_set(miller_element_set: MillerElementSet, collector: &mut NfcCollector, no_packets_this_turn: &mut usize) {
    if let Ok(frame) = miller_element_set.collect_frame() {
        if let Frame::Standard(standard_frame) = frame {
            if standard_frame.len() >= PACKET_SIZE {
                let serialized_packet = standard_frame[standard_frame.len() - PACKET_SIZE..].try_into().expect("static length, always fits");
                unsafe {GOT_FRAMES += 1}
                *no_packets_this_turn += 1;
                in_free(|peripherals| {
                    let mut external_psram = ExternalPsram{peripherals};
                    let packet = Packet::deserialize(serialized_packet);
                    unsafe {PARTICIPATED_PACKETS.push(packet.id);}
                    collector.add_packet(&mut external_psram, packet);
                });
            }
        }
    }
}
*/
pub enum NfcCollector {
    Empty,
    InProgress(DecoderMetal<AddressPsram>),
    Done(ExternalData<AddressPsram>)
}

impl NfcCollector {
    pub fn new() -> Self {
        Self::Empty
    }
    pub fn add_packet(&mut self, external_psram: &mut ExternalPsram, nfc_packet: Packet) {
        match self {
            NfcCollector::Empty => {
//                let id_clone = nfc_packet.id;
                let decoder_metal = DecoderMetal::init(external_psram, nfc_packet).unwrap();
//                let block_numbers = decoder_metal.block_numbers_for_id(id_clone);
//                if block_numbers.len() == 1 {unsafe{SINGLES += 1}}
                match decoder_metal.try_read(external_psram) {
                    None => *self = NfcCollector::InProgress(decoder_metal),
                    Some(a) => *self = NfcCollector::Done(a),
                }
            },
            NfcCollector::InProgress(decoder_metal) => {
//                let block_numbers = decoder_metal.block_numbers_for_id(nfc_packet.id);
//                if block_numbers.len() == 1 {unsafe{SINGLES += 1}}
                decoder_metal.add_packet(external_psram, nfc_packet).unwrap();
                if let Some(a) = decoder_metal.try_read(external_psram) {
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
//    ExcessData,
//    NoCompactPayload,
//    NoCompactPublicKey,
//    NoCompactSignature,
}

#[derive(Debug)]
pub struct TransferDataReceived {
    pub encoded_data: PsramAccess,
    pub companion_signature: Vec<u8>,
    pub companion_public_key: Vec<u8>,
}

pub fn process_nfc_payload(completed_collector: &ExternalData<AddressPsram>) -> Result<TransferDataReceived, NfcPayloadError> {
    let psram_data = PsramAccess {
        start_address: completed_collector.start_address.clone(),
        total_len: completed_collector.len,
    };

    let mut position = 0usize; // *relative* position in PsramAccess!

    let mut try_encoded_data = None;
    in_free(|peripherals| {
        let mut external_psram = ExternalPsram{peripherals};
        let found_compact = find_compact::<u32, PsramAccess, ExternalPsram>(&psram_data, &mut external_psram, position).unwrap(); //.map_err(|_| NfcPayloadError::NoCompactPayload)?;
        let start_address = completed_collector.start_address.try_shift(found_compact.start_next_unit).unwrap();
        try_encoded_data = Some(PsramAccess {
            start_address,
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
        let start_address = completed_collector.start_address.try_shift(found_compact.start_next_unit).unwrap();
        let signature_data = psram_read_at_address(external_psram.peripherals, start_address, found_compact.compact as usize).unwrap(); //.map_err(|_| NfcPayloadError::AccessOnSignature)?;
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
        let start_address = completed_collector.start_address.try_shift(found_compact.start_next_unit).unwrap();
        let public_key_data = psram_read_at_address(external_psram.peripherals, start_address, found_compact.compact as usize).unwrap(); //.map_err(|_| NfcPayloadError::AccessOnSignature)?;
        try_companion_public_key = Some(public_key_data);
        position = found_compact.start_next_unit + found_compact.compact as usize;
    });
    let companion_public_key = match try_companion_public_key {
        Some(a) => a,
        None => return Err(NfcPayloadError::AccessOnPublicKey),
    };

    if position != psram_data.total_len {
        panic!("after decoding position not matching total length, position: {position}, total_len: {}", psram_data.total_len);
        //Err(NfcPayloadError::ExcessData)
    }
    else {Ok(TransferDataReceived{
        encoded_data,
        companion_signature,
        companion_public_key
    })}
}
