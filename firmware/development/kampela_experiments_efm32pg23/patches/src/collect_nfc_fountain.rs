use alloc::{vec, vec::Vec};

use kampela_common::{NfcPacket, NFC_PACKET_SIZE};

pub enum NfcCollector {
    InProgress(InProgress),
    Ready(Vec<u8>),
}

impl NfcCollector {
    pub fn new(packet_data: &[u8]) -> Result<Self, ErrorPacket> {
        let packet =
            NfcPacket::from_raw_packet(packet_data).map_err(|_| ErrorPacket::PacketNotDecodable)?;
        let packet_data_len = packet.data.len() as u32;
        let optimistic_total_expected_packets = {
            if packet.payload_length % packet_data_len == 0 {
                packet.payload_length / packet_data_len
            } else {
                packet.payload_length / packet_data_len + 1
            }
        };
        let content = vec![packet.data];
        if optimistic_total_expected_packets == 1 {
            match try_fountain(&content, packet.payload_length) {
                Some(payload) => Ok(NfcCollector::Ready(payload)),
                None => Ok(NfcCollector::InProgress(InProgress {
                    content,
                    payload_length: packet.payload_length,
                    optimistic_total_expected_packets,
                })),
            }
        } else {
            Ok(NfcCollector::InProgress(InProgress {
                content,
                payload_length: packet.payload_length,
                optimistic_total_expected_packets,
            }))
        }
    }
    pub fn add_packet(
        packet_data: &[u8],
        mut in_progress: InProgress,
    ) -> Result<Self, ErrorPacket> {
        let packet =
            NfcPacket::from_raw_packet(packet_data).map_err(|_| ErrorPacket::PacketNotDecodable)?;
        if packet.payload_length != in_progress.payload_length {
            Err(ErrorPacket::FountainDifferentLength)
        } else if !in_progress.content.contains(&packet.data) {
            in_progress.content.push(packet.data);
            if in_progress.content.len() as u32 >= in_progress.optimistic_total_expected_packets {
                match try_fountain(&in_progress.content, in_progress.payload_length) {
                    Some(payload) => Ok(NfcCollector::Ready(payload)),
                    None => Ok(NfcCollector::InProgress(in_progress)),
                }
            } else {
                Ok(NfcCollector::InProgress(in_progress))
            }
        } else {
            Ok(NfcCollector::InProgress(in_progress))
        }
    }
}

fn try_fountain(content: &[Vec<u8>], payload_length: u32) -> Option<Vec<u8>> {
    let config = raptorq::ObjectTransmissionInformation::with_defaults(
        payload_length as u64,
        NFC_PACKET_SIZE,
    );
    let mut decoder = raptorq::Decoder::new(config);
    let mut result = None;
    for x in content.iter() {
        result = decoder.decode(raptorq::EncodingPacket::deserialize(x));
    }
    result
}

pub struct InProgress {
    /// packet set
    pub content: Vec<Vec<u8>>,

    /// total payload length in bytes
    pub payload_length: u32,

    /// minimal number of packets at which the decoding is tried
    pub optimistic_total_expected_packets: u32,
}

pub enum ErrorPacket {
    FountainDifferentLength,
    PacketNotDecodable,
}
