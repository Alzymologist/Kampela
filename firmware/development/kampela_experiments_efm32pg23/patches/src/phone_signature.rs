use alloc::vec::Vec;

use p256::ecdsa::{signature::Verifier, Signature, VerifyingKey};
use spki::DecodePublicKey;

use kampela_common::TransferData;

pub fn check_phone_creds(transfer_data: TransferData, stored_public_key: [u8; 33]) -> Vec<u8> {
    let signature = Signature::from_der(&transfer_data.companion_signature).unwrap();
    let verifying_key =
        VerifyingKey::from_public_key_der(&transfer_data.companion_public_key).unwrap();
    // check that the public key matches the known one
    assert_eq!(
        verifying_key.to_encoded_point(true).as_ref(),
        &stored_public_key
    );

    // check that signature is correct
    assert!(verifying_key
        .verify(&transfer_data.encoded_data, &signature)
        .is_ok());

    transfer_data.encoded_data
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_phone_verification() {
        // real values produced by app
        let transfer_data = TransferData {
            encoded_data: vec![1, 2, 3, 4],
            companion_signature: vec![
                48, 69, 2, 33, 0, 155, 236, 197, 7, 167, 251, 75, 229, 238, 144, 57, 47, 86, 212,
                89, 136, 62, 40, 172, 231, 215, 7, 37, 78, 119, 82, 225, 110, 153, 65, 91, 122, 2,
                32, 51, 132, 39, 110, 202, 241, 227, 5, 135, 10, 221, 224, 226, 17, 251, 80, 154,
                48, 208, 46, 235, 172, 245, 88, 31, 135, 114, 109, 77, 230, 112, 243,
            ],
            companion_public_key: vec![
                48, 89, 48, 19, 6, 7, 42, 134, 72, 206, 61, 2, 1, 6, 8, 42, 134, 72, 206, 61, 3, 1,
                7, 3, 66, 0, 4, 231, 64, 200, 164, 0, 35, 127, 25, 191, 7, 173, 153, 131, 110, 129,
                105, 56, 64, 161, 204, 93, 220, 1, 224, 190, 55, 170, 57, 4, 206, 35, 111, 19, 97,
                123, 206, 117, 104, 156, 174, 153, 22, 86, 212, 135, 101, 22, 38, 88, 26, 195, 62,
                179, 37, 15, 107, 63, 225, 160, 135, 70, 238, 180, 54,
            ],
        };
        let stored_public_key = [
            2, 231, 64, 200, 164, 0, 35, 127, 25, 191, 7, 173, 153, 131, 110, 129, 105, 56, 64,
            161, 204, 93, 220, 1, 224, 190, 55, 170, 57, 4, 206, 35, 111,
        ];
        assert_eq!(
            check_phone_creds(transfer_data, stored_public_key),
            [1, 2, 3, 4]
        );
    }
}
