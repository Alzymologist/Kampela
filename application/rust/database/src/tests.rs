use crate::process_input::Encryption;
use crate::storage::{MetadataStorage, SpecsValue};

#[test]
fn good_specs_processing_1() {
    let payload_hex = std::fs::read_to_string("for_tests/add_specs_rococo-sr25519_Alice-sr25519.txt").unwrap();
    let payload = hex::decode(payload_hex).unwrap();
    let specs_result = SpecsValue::from_payload_prelude_cut(&payload[3..], &Encryption::Sr25519);
    assert!(specs_result.is_ok());
}

#[test]
fn good_meta_processing() {
    let payload_hex = std::fs::read_to_string("for_tests/load_metadata_westendV9122_Alice-sr25519.txt").unwrap();
    let payload = hex::decode(payload_hex).unwrap();
    let meta_result = MetadataStorage::from_payload_prelude_cut(&payload[3..], &Encryption::Sr25519);
    assert!(meta_result.is_ok());
}
