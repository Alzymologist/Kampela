//! NFC messages are sent from companion into Kampela.
//!
//! Kampela must not produce any signal for companion to figure that the
//! message was successfully passed. Fountain is stopped by user on the
//! companion side when Kampela shows on screen that the message is received.
use raptorq::{Encoder, ObjectTransmissionInformation};
use std::convert::TryFrom;

use kampela_common::{NfcPacket, KAMPELA_DECODER_MEMORY, NFC_PAYLOAD_SIZE};

use crate::error::ErrorCompanion;

/// Form a set of `Vec<u8>` limited length NFC payloads from `&[u8]` input
pub fn pack_nfc(input: &[u8]) -> Result<Vec<NfcPacket>, ErrorCompanion> {
    // Input length. Reasonable input data is expected to fit in `u32`.
    let payload_length = match u32::try_from(input.len()) {
        Ok(a) => a,
        Err(_) => return Err(ErrorCompanion::TooLargeInputForNFC),
    };

    // Number of repair packets.
    // Currently roughly equal to number of core packets.
    let repair_packets_per_block: u32 = {
        if payload_length <= NFC_PAYLOAD_SIZE as u32 {
            0
        } else {
            payload_length / NFC_PAYLOAD_SIZE as u32
        }
    };

    // ObjectTransmissionInformation, compatible with Kampela memory abilities
    let object_transmission_information =
        ObjectTransmissionInformation::generate_encoding_parameters_exposed(
            payload_length as u64,
            NFC_PAYLOAD_SIZE,
            KAMPELA_DECODER_MEMORY,
        );

    // Raptorq Encoder, using ObjectTransmissionInformation
    let raptor_encoder = Encoder::new(input, object_transmission_information);

    // Deserialize and collect packets
    Ok(raptor_encoder
        .get_encoded_packets(repair_packets_per_block)
        .iter()
        .map(|x| {
            NfcPacket::construct(
                payload_length,
                x.serialize()
                    .try_into()
                    .expect("length statically determined by NFC_PAYLOAD_SIZE and upstream"),
            )
        })
        .collect::<Vec<NfcPacket>>())
}
