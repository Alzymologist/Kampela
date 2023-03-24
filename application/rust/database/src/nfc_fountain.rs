//! NFC messages are sent from companion into Kampela.
//!
//! Kampela must not produce any signal for companion to figure that the
//! message was successfully passed. Fountain is stopped by user on the
//! companion side when Kampela shows on screen that the message is received.

use std::convert::TryFrom;

use kampela_common::{NfcPacket, NFC_PACKET_SIZE};

use crate::error::ErrorCompanion;

/// Form a set of `Vec<u8>` limited length NFC payloads from `&[u8]` input
pub fn pack_nfc(input: &[u8]) -> Result<Vec<Vec<u8>>, ErrorCompanion> {
    // Input length. Reasonable input data is expected to fit in `u32`.
    let payload_length = match u32::try_from(input.len()) {
        Ok(a) => a,
        Err(_) => return Err(ErrorCompanion::TooLargeInputForNFC),
    };

    // Number of repair packets.
    // Currently roughly equal to number of core packets.
    let repair_packets_per_block: u32 = {
        if payload_length <= NFC_PACKET_SIZE as u32 {
            0
        } else {
            payload_length / NFC_PACKET_SIZE as u32
        }
    };

    // Raptorq Encoder, with defaults
    let raptor_encoder = raptorq::Encoder::with_defaults(input, NFC_PACKET_SIZE);

    // Deserialize and collect packets
    Ok(raptor_encoder
        .get_encoded_packets(repair_packets_per_block)
        .iter()
        .map(|x| {
            NfcPacket {
                payload_length,
                data: x.serialize(),
            }
            .as_raw_packet()
        })
        .collect::<Vec<Vec<u8>>>())
}
