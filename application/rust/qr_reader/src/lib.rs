#![deny(unused_crate_dependencies)]
#![deny(rustdoc::broken_intra_doc_links)]

use std::convert::TryInto;
use std::sync::{Arc, RwLock};

/// Fountain QR code payload length upper limit.
///
/// Maximum allowed payload length for fountain QR code is below `0x80000000`,
/// i.e. 2_147_483_648 bytes. For comparison, current metadata QR payloads
/// (largest payloads transferred) have lengths under 400_000 bytes.
///
/// There are two types of multiframe QR codes: fountain QR codes and legacy
/// multiframe QR codes, and static QR codes.
///
/// Fountain QR codes begin with 4-bytes indicator of payload length (these 4
/// bytes are big endian `u32` length of expected decoded payload in bytes plus
/// [`FOUNTAIN_LIMIT`]). This way the first byte of a fountain QR code payload
/// always exceeds or is equal to [`FOUNTAIN_MARKER`].
///
/// Legacy multiframe QR codes have first payload byte `0x00`, thus clearly
/// distinct from fountains.
///
/// Expected static QR codes start immediately with the payload content, i.e.
/// `0x53..`, also distinct from fountains.
///
/// To distinguish between QR code variants on scanning, the first byte is used.
///
/// To allow distinguishing between fountain and legacy multiframe QR codes when
/// multiframe payloads are read, the length of payload (i.e. a value under
/// `0x80000000`) remains as is for legacy multiframe and has addition of
/// `0x80000000` for fountain QR code. The resulting value always fits in `u32`,
/// that gets into first 4 bytes of each frame. When scanner reads a frame, if
/// the value is below `0x80000000` it gets processed as legacy multiframe, if
/// equal or above - as fountain QR code.
pub const FOUNTAIN_LIMIT: u32 = 0x80000000;

/// Marker to distinguish fountain QR code by the first byte.
///
/// First byte of each fountain QR code frame is the first byte of `u32` payload
/// length (big endian).
///
/// If, within allowed length limits, the first byte exceeds
/// [`FOUNTAIN_MARKER`], the QR code is the fountain one, i.e.
/// [`FOUNTAIN_LIMIT`] was added to the payload length.
pub const FOUNTAIN_MARKER: u8 = 0b10000000;

/// Data chunk size for fountain QR code.
pub const CHUNK_SIZE: u16 = 1072;

#[derive(Debug, Eq, PartialEq, thiserror::Error)]
/// QR code reader errors.
pub enum ErrorQr {
    #[error("Empty frame.")]
    EmptyFrame,

    #[error(
        "While collecting fountain QR code, encountered a frame for different payload length."
    )]
    FountainDifferentLength,

    #[error("Frame appears to be a fountain QR code frame, but payload {} is too short to get payload length.", show_raw_payload(raw_frame))]
    FountainFrameTooShort { raw_frame: Vec<u8> },

    #[error("Collecting fountain QR code was interrupted by a legacy multiframe QR frame.")]
    FountainInterruptedByLegacy,

    #[error("Collecting fountain QR code was interrupted by a static QR frame.")]
    FountainInterruptedByStatic,

    #[error(
        "Frame appears to be a fountain QR code frame, but payload {} contains empty packet.",
        show_raw_payload(raw_frame)
    )]
    FountainPacketEmpty { raw_frame: Vec<u8> },

    #[error(
        "While collecting legacy multiframe QR code, encountered a frame from a different one."
    )]
    LegacyDifferentLength,

    #[error("Collecting legacy multiframe QR code was interrupted by a fountain QR frame.")]
    LegacyInterruptedByFountain,

    #[error("Collecting legacy multiframe QR code was interrupted by a static QR frame.")]
    LegacyInterruptedByStatic,

    #[error("While collecting legacy multiframe QR code, encountered two different frames with same order.")]
    LegacySameOrderDifferentContent,

    #[error("Frame appears to be a legacy multiframe QR code frame, but payload {} is too short to get total number of frames.", show_raw_payload(raw_frame))]
    LegacyTooShortNumberOfFrames { raw_frame: Vec<u8> },

    #[error("Frame appears to be a legacy multiframe QR code frame, but payload {} is too short to get frame order.", show_raw_payload(raw_frame))]
    LegacyTooShortOrder { raw_frame: Vec<u8> },

    #[error("Frame appears to be a legacy multiframe QR code frame, but frame order {} is too high for expected number of frames {}.", order, number_of_frames)]
    LegacyOrderTooHigh { order: u16, number_of_frames: u16 },

    #[error("Frame appears to be a legacy multiframe QR code frame, but payload {} corresponds to 0 total frames.", show_raw_payload(raw_frame))]
    LegacyZeroFrames { raw_frame: Vec<u8> },

    #[error("Lock is poisoned.")]
    PoisonedLock,
}

/// Display questionable QR code payload, for error display.
fn show_raw_payload(raw_frame: &[u8]) -> String {
    if raw_frame.len() > 8 {
        format!(
            "{}..{}",
            hex::encode(&raw_frame[..4]),
            hex::encode(&raw_frame[raw_frame.len() - 4..])
        )
    } else {
        hex::encode(raw_frame)
    }
}

/// Collected and processed frames interacting with the outside code through
/// `uniffi`
#[derive(Debug)]
pub struct Collection {
    collection: RwLock<CollectionBody>,
}

impl Collection {
    /// Make new [`Collection`].
    pub fn new() -> Self {
        Collection {
            collection: RwLock::new(CollectionBody::Empty),
        }
    }

    /// Clean existing [`Collection`].
    pub fn clean(self: &Arc<Self>) -> Result<(), ErrorQr> {
        let mut collection = self.collection.write().map_err(|_| ErrorQr::PoisonedLock)?;
        *collection = CollectionBody::Empty;
        Ok(())
    }

    /// Process new frame and modify [`Collection`]. Outputs optional final
    /// result, indicating to UI that it is time to proceed.
    pub fn process_frame(self: &Arc<Self>, raw_frame: Vec<u8>) -> Result<Payload, ErrorQr> {
        let mut collection = self.collection.write().map_err(|_| ErrorQr::PoisonedLock)?;
        match &*collection {
            CollectionBody::Empty => {
                *collection = CollectionBody::init(raw_frame)?;
                if let CollectionBody::Ready { payload } = &*collection {
                    Ok(Payload {
                        payload: Some(payload.to_owned()),
                    })
                } else {
                    Ok(Payload { payload: None })
                }
            }
            CollectionBody::Ready { .. } => Ok(Payload { payload: None }),
            CollectionBody::NotReady { multi } => {
                *collection = CollectionBody::add_frame(raw_frame, multi.to_owned())?;
                if let CollectionBody::Ready { payload } = &*collection {
                    Ok(Payload {
                        payload: Some(payload.to_owned()),
                    })
                } else {
                    Ok(Payload { payload: None })
                }
            }
        }
    }

    pub fn frames(&self) -> Result<Option<Frames>, ErrorQr> {
        let collection = self.collection.read().map_err(|_| ErrorQr::PoisonedLock)?;
        match &*collection {
            CollectionBody::Empty => Ok(None),
            CollectionBody::Ready { .. } => Ok(None),
            CollectionBody::NotReady { multi } => Ok(Some(Frames {
                current: multi.currently_collected_frames(),
                total: multi.total_expected_frames(),
            })),
        }
    }
}

impl Default for Collection {
    fn default() -> Self {
        Self::new()
    }
}

/// Collected and processed frames
#[derive(Debug, PartialEq)]
enum CollectionBody {
    /// Initiated, no frames yet received
    Empty,

    /// No more frames needed, result is ready
    Ready { payload: Vec<u8> },

    /// Need more frames, for multiframe QRs only
    NotReady { multi: Multi },
}

#[derive(Clone, Debug, PartialEq)]
/// Partially collected and processed multiframe QR data
enum Multi {
    /// Fountain QR
    Fountain {
        /// packet set
        content: Vec<Vec<u8>>,

        /// total payload length in bytes
        payload_length: u32,

        /// minimal number of frames at which the decoding is tried
        optimistic_total_expected_frames: u32,
    },

    /// Legacy multiframe QR
    Legacy {
        /// collected frames content
        content: Vec<LegacyMultiContent>,

        /// total number of frames, all must be collected
        total_expected_frames: u16,
    },
}

#[derive(Clone, Debug, PartialEq)]
/// Individual frame content for legacy multiframe QRs
struct LegacyMultiContent {
    /// order [0..total_expected_frames)
    order: u16,

    /// corresponding data
    data: Vec<u8>,
}

/// Object to move output through uniffi
pub struct Payload {
    pub payload: Option<Vec<u8>>,
}

/// Object to move number of frames through uniffi
pub struct Frames {
    pub current: u32,
    pub total: u32,
}

impl CollectionBody {
    /// Initiate collection with the first frame.
    fn init(raw_frame: Vec<u8>) -> Result<Self, ErrorQr> {
        match Frame::from_raw(&raw_frame)? {
            Frame::Fountain {
                frame_content,
                frame_payload_length,
            } => {
                if frame_content.is_empty() {
                    return Err(ErrorQr::FountainPacketEmpty { raw_frame });
                }

                let frame_content_len = frame_content.len() as u32;

                // this is minimal number of frames at which the Raptor decoder
                // is tried
                //
                // expected number of frames displayed to user is higher by one
                let optimistic_total_expected_frames = {
                    if frame_payload_length % frame_content_len == 0 {
                        frame_payload_length / frame_content_len
                    } else {
                        frame_payload_length / frame_content_len + 1
                    }
                };
                let content = vec![frame_content];
                if optimistic_total_expected_frames == 1 {
                    match try_fountain(&content, frame_payload_length) {
                        Some(payload) => Ok(CollectionBody::Ready { payload }),
                        None => Ok(CollectionBody::NotReady {
                            multi: Multi::Fountain {
                                content,
                                payload_length: frame_payload_length,
                                optimistic_total_expected_frames,
                            },
                        }),
                    }
                } else {
                    Ok(CollectionBody::NotReady {
                        multi: Multi::Fountain {
                            content,
                            payload_length: frame_payload_length,
                            optimistic_total_expected_frames,
                        },
                    })
                }
            }
            Frame::LegacyMulti {
                frame_content,
                frame_total_expected_frames,
            } => {
                if frame_total_expected_frames == 1 {
                    Ok(CollectionBody::Ready {
                        payload: frame_content.data,
                    })
                } else {
                    Ok(CollectionBody::NotReady {
                        multi: Multi::Legacy {
                            content: vec![frame_content],
                            total_expected_frames: frame_total_expected_frames,
                        },
                    })
                }
            }
            Frame::Static => Ok(CollectionBody::Ready { payload: raw_frame }),
        }
    }

    /// Add one more frame to already initiated, but not yet completed
    /// collection, and check if collection becomes completed.
    fn add_frame(raw_frame: Vec<u8>, collection_not_ready: Multi) -> Result<Self, ErrorQr> {
        match Frame::from_raw(&raw_frame)? {
            Frame::Fountain {
                frame_content,
                frame_payload_length,
            } => match collection_not_ready {
                Multi::Fountain {
                    mut content,
                    payload_length,
                    optimistic_total_expected_frames,
                } => {
                    if frame_payload_length != payload_length {
                        Err(ErrorQr::FountainDifferentLength)
                    } else if !content.contains(&frame_content) {
                        content.push(frame_content);
                        Ok(frame_added_to_fountain(
                            content,
                            payload_length,
                            optimistic_total_expected_frames,
                        ))
                    } else {
                        Ok(CollectionBody::NotReady {
                            multi: Multi::Fountain {
                                content,
                                payload_length,
                                optimistic_total_expected_frames,
                            },
                        })
                    }
                }
                Multi::Legacy { .. } => Err(ErrorQr::LegacyInterruptedByFountain),
            },
            Frame::LegacyMulti {
                frame_content,
                frame_total_expected_frames,
            } => match collection_not_ready {
                Multi::Fountain { .. } => Err(ErrorQr::FountainInterruptedByLegacy),
                Multi::Legacy {
                    content,
                    total_expected_frames,
                } => add_frame_to_legacy(
                    frame_content,
                    frame_total_expected_frames,
                    content,
                    total_expected_frames,
                ),
            },
            Frame::Static => match collection_not_ready {
                Multi::Fountain { .. } => Err(ErrorQr::FountainInterruptedByStatic),
                Multi::Legacy { .. } => Err(ErrorQr::LegacyInterruptedByStatic),
            },
        }
    }
}

impl Multi {
    /// Number of good different frames already collected.
    fn currently_collected_frames(&self) -> u32 {
        match self {
            Multi::Fountain { content, .. } => content.len() as u32,
            Multi::Legacy { content, .. } => content.len() as u32,
        }
    }

    /// Display pessimistic number of frames needed to process QR sequence, for
    /// UI.
    ///
    /// In fountain QR decoding the decoding algorithm is probabilistic, see
    /// documentation of the `raptorq` crate and
    /// <https://datatracker.ietf.org/doc/html/rfc6330>.
    ///
    /// Rephrasing from there, the probability to decode message with all source
    /// packets and h additional repair packets is 1 - 1/256^(h+1).
    ///
    /// Value `optimistic_total_expected_frames` equals to number of source
    /// packets.
    ///
    /// If there are no additional packets, probability is ~ 0.99609.
    ///
    /// If one additional packet is added, it is ~ 0.99998.
    ///
    /// It was decided to add one additional packet in the printed estimate, so
    /// that the user expectations are lower.
    ///
    /// In legacy multiframe QR codes all existing frames must be collected.
    fn total_expected_frames(&self) -> u32 {
        match self {
            Multi::Fountain {
                optimistic_total_expected_frames,
                ..
            } => *optimistic_total_expected_frames + 1,
            Multi::Legacy {
                total_expected_frames,
                ..
            } => *total_expected_frames as u32,
        }
    }
}

/// Individual QR code processed.
enum Frame {
    /// Supposedly a part of fountain QR.
    Fountain {
        /// Packet, one of many to be fed into `raptorq` decoder.
        frame_content: Vec<u8>,

        /// Decoded payload length, in bytes.
        frame_payload_length: u32,
    },

    /// Supposedly a part of legacy multiframe QR.
    LegacyMulti {
        /// Individual frame [`LegacyMultiContent`].
        frame_content: LegacyMultiContent,

        /// Total number of frames, will need them all to decode.
        frame_total_expected_frames: u16,
    },

    /// Static QR.
    Static,
}

impl Frame {
    /// Process raw payload and get the [`Frame`].
    fn from_raw(raw: &[u8]) -> Result<Self, ErrorQr> {
        let first_byte = match raw.first() {
            Some(a) => *a,
            None => return Err(ErrorQr::EmptyFrame),
        };
        if first_byte >= FOUNTAIN_MARKER {
            let frame_payload_length = match raw.get(..4) {
                Some(a) => {
                    let payload_length_piece: [u8; 4] =
                        a.try_into().expect("constant size, always fits");
                    u32::from_be_bytes(payload_length_piece) - FOUNTAIN_LIMIT
                }
                None => {
                    return Err(ErrorQr::FountainFrameTooShort {
                        raw_frame: raw.to_vec(),
                    })
                }
            };
            let frame_content = raw[4..].to_vec();
            Ok(Frame::Fountain {
                frame_content,
                frame_payload_length,
            })
        } else if first_byte == 0 {
            let frame_total_expected_frames = match raw.get(1..3) {
                Some(a) => {
                    let total_expected_frames_piece: [u8; 2] =
                        a.try_into().expect("constant size, always fits");
                    u16::from_be_bytes(total_expected_frames_piece)
                }
                None => {
                    return Err(ErrorQr::LegacyTooShortNumberOfFrames {
                        raw_frame: raw.to_vec(),
                    });
                }
            };
            if frame_total_expected_frames == 0 {
                return Err(ErrorQr::LegacyZeroFrames {
                    raw_frame: raw.to_vec(),
                });
            }
            let order = match raw.get(3..5) {
                Some(a) => {
                    let order_piece: [u8; 2] = a.try_into().expect("constant size, always fits");
                    u16::from_be_bytes(order_piece)
                }
                None => {
                    return Err(ErrorQr::LegacyTooShortOrder {
                        raw_frame: raw.to_vec(),
                    })
                }
            };
            if order >= frame_total_expected_frames {
                return Err(ErrorQr::LegacyOrderTooHigh {
                    order,
                    number_of_frames: frame_total_expected_frames,
                });
            }
            let data = raw[5..].to_vec();
            let frame_content = LegacyMultiContent { order, data };
            Ok(Frame::LegacyMulti {
                frame_content,
                frame_total_expected_frames,
            })
        } else {
            Ok(Frame::Static)
        }
    }
}

/// Helper function, to process the newly updated fountain QR data.
fn frame_added_to_fountain(
    content: Vec<Vec<u8>>,
    payload_length: u32,
    optimistic_total_expected_frames: u32,
) -> CollectionBody {
    if content.len() as u32 >= optimistic_total_expected_frames {
        match try_fountain(&content, payload_length) {
            Some(payload) => CollectionBody::Ready { payload },
            None => CollectionBody::NotReady {
                multi: Multi::Fountain {
                    content,
                    payload_length,
                    optimistic_total_expected_frames,
                },
            },
        }
    } else {
        CollectionBody::NotReady {
            multi: Multi::Fountain {
                content,
                payload_length,
                optimistic_total_expected_frames,
            },
        }
    }
}

/// Try to assemble the fountain data. `Some(_)` if successful, `None` if not
/// yet enough data is gathered.
///
/// Warning. This panics in upstream (`raptorq` crate) if the content is not
/// really a part of processable fountain data. For example, mock frames like
/// `vec![128, 0, 0, 0, 5]` will panic here, as decoder addresses parts of
/// vector directly.
fn try_fountain(content: &[Vec<u8>], payload_length: u32) -> Option<Vec<u8>> {
    let config =
        raptorq::ObjectTransmissionInformation::with_defaults(payload_length as u64, CHUNK_SIZE);
    let mut decoder = raptorq::Decoder::new(config);
    let mut result = None;
    for x in content.iter() {
        result = decoder.decode(raptorq::EncodingPacket::deserialize(x));
    }
    result
}

/// Adds a frame to legacy multiframe QR data, finalizes the collection if all
/// frames assembled.
fn add_frame_to_legacy(
    frame_content: LegacyMultiContent,
    frame_total_expected_frames: u16,
    mut content: Vec<LegacyMultiContent>,
    total_expected_frames: u16,
) -> Result<CollectionBody, ErrorQr> {
    if frame_total_expected_frames != total_expected_frames {
        return Err(ErrorQr::LegacyDifferentLength);
    }
    let mut is_frame_in_set = false;
    for element in content.iter() {
        if element.order == frame_content.order {
            if element.data != frame_content.data {
                return Err(ErrorQr::LegacySameOrderDifferentContent);
            }
            is_frame_in_set = true;
            break;
        }
    }
    if !is_frame_in_set {
        content.push(frame_content);
        // `as u16`, because there could be no more elements than
        // `total_expected_frames` (only correct order gets in, order is checked
        // to be below `total_expected_frames`)
        if content.len() as u16 == total_expected_frames {
            let mut payload: Vec<u8> = Vec::new();
            content.sort_by(|a, b| a.order.cmp(&b.order));
            for element in content.iter() {
                payload.extend_from_slice(&element.data);
            }
            Ok(CollectionBody::Ready { payload })
        } else {
            Ok(CollectionBody::NotReady {
                multi: Multi::Legacy {
                    content,
                    total_expected_frames,
                },
            })
        }
    } else {
        Ok(CollectionBody::NotReady {
            multi: Multi::Legacy {
                content,
                total_expected_frames,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_no_panic() {
        assert_eq!(
            CollectionBody::init(vec![]).unwrap_err(),
            ErrorQr::EmptyFrame
        );
        assert_eq!(
            CollectionBody::init(vec![128]).unwrap_err(),
            ErrorQr::FountainFrameTooShort {
                raw_frame: vec![128]
            }
        );
        assert_eq!(
            CollectionBody::init(vec![128, 155, 100, 108]).unwrap_err(),
            ErrorQr::FountainPacketEmpty {
                raw_frame: vec![128, 155, 100, 108]
            }
        );
        assert_eq!(
            CollectionBody::init(vec![0]).unwrap_err(),
            ErrorQr::LegacyTooShortNumberOfFrames { raw_frame: vec![0] }
        );
        assert_eq!(
            CollectionBody::init([0; 3].to_vec()).unwrap_err(),
            ErrorQr::LegacyZeroFrames {
                raw_frame: [0; 3].to_vec()
            }
        );
        assert_eq!(
            CollectionBody::init(vec![0, 1, 0, 5]).unwrap_err(),
            ErrorQr::LegacyTooShortOrder {
                raw_frame: vec![0, 1, 0, 5]
            }
        );
        assert_eq!(
            CollectionBody::init(vec![0, 0, 5, 0, 8]).unwrap_err(),
            ErrorQr::LegacyOrderTooHigh {
                order: 8,
                number_of_frames: 5
            }
        );
    }

    #[test]
    fn init_correct_outcome() {
        // single frame in legacy multiframe format
        assert_eq!(
            CollectionBody::init(vec![0, 0, 1, 0, 0, 3, 14, 15, 92, 6, 54]).unwrap(),
            CollectionBody::Ready {
                payload: vec![3, 14, 15, 92, 6, 54]
            },
        );

        // single frame in fountain multiframe format
        assert_eq!(
            CollectionBody::init(
                [
                    vec![128, 0, 0, 7, 0, 0, 0, 0, 14, 25, 26, 17, 33, 21, 60],
                    [0; 1065].to_vec()
                ]
                .concat()
            )
            .unwrap(),
            CollectionBody::Ready {
                payload: vec![14, 25, 26, 17, 33, 21, 60]
            },
        );

        // single frame in fountain multiframe format, real payload is `[100; 1072].to_vec()`
        assert_eq!(
            CollectionBody::init([vec![128, 0, 4, 48, 0, 0, 0, 0], [100; 1072].to_vec()].concat())
                .unwrap(),
            CollectionBody::Ready {
                payload: [100; 1072].to_vec()
            },
        );

        // first frame in founrain multiframe format, real payload is `[100; 2144].to_vec()`
        assert_eq!(
            CollectionBody::init([vec![128, 0, 8, 96, 0, 0, 0, 0], [100; 1072].to_vec()].concat())
                .unwrap(),
            CollectionBody::NotReady {
                multi: Multi::Fountain {
                    content: vec![[[0; 4].to_vec(), [100; 1072].to_vec()].concat()],
                    payload_length: 2144,
                    optimistic_total_expected_frames: 2,
                }
            }
        );
    }

    #[test]
    fn show_raw_payload_correctly() {
        let raw_frame = [0, 0, 5, 0, 1];
        assert_eq!(show_raw_payload(&raw_frame), "0000050001");
        let raw_frame = [0, 0, 5, 0, 1, 3, 14];
        assert_eq!(show_raw_payload(&raw_frame), "0000050001030e");
        let raw_frame = [0, 0, 5, 0, 1, 3, 14, 15];
        assert_eq!(show_raw_payload(&raw_frame), "0000050001030e0f");
        let raw_frame = [0, 0, 5, 0, 1, 3, 14, 15, 92];
        assert_eq!(show_raw_payload(&raw_frame), "00000500..030e0f5c");
        let raw_frame = [0, 0, 5, 0, 1, 3, 14, 15, 92, 6, 54];
        assert_eq!(show_raw_payload(&raw_frame), "00000500..0f5c0636");
    }
}
