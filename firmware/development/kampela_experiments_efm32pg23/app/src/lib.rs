//! Testing basic functionality with `efm32pg23` chip.
//!
//! Operations used here are based on
//!
//! - [reference manual](https://www.silabs.com/documents/public/reference-manuals/efm32pg23-rm.pdf)
//! - [devboard user guide](https://www.silabs.com/documents/public/user-guides/ug515-efm32pg23-brd2504a-user-guide.pdf)
//! - [official API docs](https://docs.silabs.com/gecko-platform/latest/emlib/api/efm32xg23/modules)
//! - [published official open source SDK in C](https://github.com/SiliconLabs/gecko_sdk/tree/gsdk_4.2/platform)

#![no_std]

extern crate alloc;
extern crate core;

use alloc::{vec, vec::Vec};
use k256::{
    ecdsa::{SigningKey, signature::hazmat::PrehashVerifier, VerifyingKey},
};
use rand_core::RngCore;
use schnorrkel::{
    context::attach_rng,
    derive::{ChainCode, Derivation},
    signing_context, ExpansionMode, MiniSecretKey,
};
use sp_core::{crypto::DeriveJunction, ed25519, Pair, hashing::blake2_256, Encode};

use efm32pg23_fix::Peripherals;
use patches::{cut_derivations, entropy_to_big_seed, entropy_to_phrase, phrase_to_entropy};

pub mod led_blinker;

pub mod se_aes_gcm;
use se_aes_gcm::{
    aes_gcm_decrypt, aes_gcm_encrypt, create_key, AAD_LEN, IV_LEN, KEY_BUFFER, KEY_BUFFER_LEN,
};

pub mod se_command;
use se_command::{se_off, se_on, RxError};

pub mod se_rng;
use se_rng::{random_with_length, SeRng};

/// Ticks counter.
pub static mut COUNT: u32 = 0;

/// Make visible delay. For blinker. Input is delay time in ms.
pub fn visible_delay(ticks_ms: u32) {
    unsafe {
        let end = COUNT.wrapping_add(ticks_ms);

        while end > COUNT {
            cortex_m::asm::wfi();
        }
    }
}

/// Hardcoded iv, eventually will be stored in flash.
pub const IV: [u8; IV_LEN] = [1; IV_LEN];

/// Hardcoded pincode, eventually will be entered by user.
pub const PINCODE_CORRECT: [u8; AAD_LEN] = [2; AAD_LEN];

/// Incorrect pincode.
pub const PINCODE_INCORRECT: [u8; AAD_LEN] = [7; AAD_LEN];

/// Hardcoded secret. Eventually will be generated on Kampela or entered by
/// user.
pub fn get_secret() -> Vec<u8> {
    vec![3; 28]
}

/// Try run basic AES GSM related sequence.
///
/// - create device key,
/// - encrypt user secret,
/// - successfully decrypt user secret with good pincode,
/// - fail to decrypt user secret with bad pincode
pub fn test_aes_gcm_sequence(peripherals: &mut Peripherals) {
    se_on(peripherals);

    // Create key and write it (wrapped) in static `KEY_BUFFER`.
    // Currently `KEY_BUFFER` is a static in RAM.
    // Later this must be changed with writing to flash.
    create_key(peripherals).unwrap();

    // `KEY_BUFFER` was initiated with `0`, check that it was actually written.
    unsafe {
        assert_ne!(KEY_BUFFER, [0; KEY_BUFFER_LEN]);
    }

    let secret = get_secret();

    // encrypting
    let out_encoded = aes_gcm_encrypt(peripherals, PINCODE_CORRECT, IV, secret).unwrap();

    // decrypting with correct pincode
    let out_decoded = aes_gcm_decrypt(peripherals, &out_encoded, PINCODE_CORRECT, IV).unwrap();

    let original_secret = get_secret();

    assert_eq!(original_secret.len(), out_decoded.len);
    assert_eq!(original_secret, out_decoded.data[..out_decoded.len]);

    // decoding with incorrect pincode
    let error = aes_gcm_decrypt(peripherals, &out_encoded, PINCODE_INCORRECT, IV).unwrap_err();

    assert_eq!(error, RxError::InvalidSignature);

    se_off(peripherals);
}

/// Length of entropy required to generate 24 word seed phrase.
pub const ENTROPY_LEN: usize = 32;

/// Try basic RNG functionality.
///
/// - create `u8` sequences to be used as entropy,
/// - check `RngCore` methods with `SeRng`
pub fn test_rng_interaction(peripherals: &mut Peripherals) {
    se_on(peripherals);

    let ent1 = random_with_length(peripherals, ENTROPY_LEN).unwrap();
    let ent2 = random_with_length(peripherals, ENTROPY_LEN).unwrap();
    assert_ne!(ent1, ent2);

    let _val = SeRng { peripherals }.next_u32();
    let _val = SeRng { peripherals }.next_u64();

    let mut data = vec![0; 19];
    SeRng { peripherals }.fill_bytes(&mut data);
    assert_ne!(data, &[0; 19]);

    se_off(peripherals);
}

/// Make seed phrase from entropy and entropy from seed phrase.
///
/// Entropy could be transformed into seed phrase and seed phrase could be
/// reversed into entropy (this is the functionality of `tiny-bip39` crate,
/// rewritten here to accomodate `no_std`).
///
/// User will see and remember the seed phrase.
///
/// Kampela will store the entropy.
pub fn test_entropy_seed(peripherals: &mut Peripherals) {
    se_on(peripherals);

    // generate entropy
    // same procedure will be used when user generates seed phrase on Kampela
    let ent1 = random_with_length(peripherals, ENTROPY_LEN).unwrap();

    // get phrase from entropy
    let phrase = entropy_to_phrase(&ent1).unwrap();

    // recover entropy from phrase
    let ent2 = phrase_to_entropy(&phrase).unwrap();

    assert_eq!(ent1, ent2);

    se_off(peripherals);
}

/// Make seed from entropy, make pairs from seed, try deriving, signing,
/// signature verification.
///
/// Entropy could be transformed into 64-byte seed, irreversibly and with an
/// optional password (this is the functionality of `substrate-bip39` crate,
/// rewritten here to accomodate `no_std`). First 32 bytes of the big seed
/// are used as seed in `sp_core`, to generate key pairs.
///
/// For Ed25519 encryption, normal `sp_core` functionality works.
///
/// For Sr25519 encryption, an override with directly used `schnorrkel` crate
/// functionality is necessary.
/// `sp_core` does not support introducing external rng, which is required for
/// soft derivations and signing.
///
/// For Ecdsa encryption, normal `sp_core` functionality works except for
/// signature verifying (the latter fails on compilation for now, apparently
/// requiring too much flash).
pub fn test_pairs_derive_sign_verify(peripherals: &mut Peripherals) {
    se_on(peripherals);

    let ent = random_with_length(peripherals, ENTROPY_LEN).unwrap();

    let big_seed = entropy_to_big_seed(&ent, "").unwrap();

    let mini_secret_bytes = &big_seed[..32];
    let msg = [0; 100];

    // sr25519
    {
        let pair = MiniSecretKey::from_bytes(mini_secret_bytes)
            .unwrap()
            .expand_to_keypair(ExpansionMode::Ed25519);

        // soft derivation
        let junction = DeriveJunction::soft("alice");
        let _pair_derived = pair
            .derived_key_simple_rng(ChainCode(*junction.inner()), [], SeRng { peripherals })
            .0;

        // hard derivation
        let junction = DeriveJunction::hard("alice");
        let _pair_derived = pair
            .hard_derive_mini_secret_key(Some(ChainCode(*junction.inner())), b"")
            .0
            .expand_to_keypair(ExpansionMode::Ed25519);

        const SIGNING_CTX: &[u8] = b"substrate";
        let context = signing_context(SIGNING_CTX);
        let signature = pair.sign(attach_rng(context.bytes(&msg), SeRng { peripherals }));

        assert!(pair
            .public
            .verify_simple(SIGNING_CTX, &msg, &signature)
            .is_ok());
    }

    // ed25519
    {
        let pair = ed25519::Pair::from_seed_slice(mini_secret_bytes).unwrap();
        assert!(pair
            .derive(cut_derivations("//1//3").unwrap().into_iter(), None)
            .is_ok());
        let signature = pair.sign(&msg);
        assert!(ed25519::Pair::verify(&signature, msg, &pair.public()));
    }

    // ecdsa, with k256 crate, result fully compatible with sp_core
    {
        let signing_key = SigningKey::from_bytes(mini_secret_bytes).unwrap();
        let verifying_key = VerifyingKey::from(&signing_key);

        let message_blake2_hash = blake2_256(&msg);
        let (signature, recid) = signing_key.sign_prehash_recoverable(&message_blake2_hash).unwrap();
        let _full_signature: [u8;65] = [signature.to_bytes().as_slice().to_vec(), vec![recid.to_byte()]].concat().try_into().unwrap();
        assert!(verifying_key.verify_prehash(&message_blake2_hash, &signature).is_ok());

        // hard derivation
        let junction = DeriveJunction::hard("alice");
        let _signing_key_derived = SigningKey::from_bytes(&("Secp256k1HDKD", mini_secret_bytes, junction.inner()).using_encoded(sp_core::hashing::blake2_256)).unwrap();

/*
        // this is direct use of ecdsa from sp_core, it compiles except verification part, that demands too much memory
        let pair = ecdsa::Pair::from_seed_slice(mini_secret_bytes).unwrap();
        assert!(pair
            .derive(cut_derivations("//alice//1").unwrap().into_iter(), None)
            .is_ok());
        let _signature = pair.sign(&msg);
        // assert!(ecdsa::Pair::verify(&signature, &msg, &pair.public())); <-- this does not compile and claims insufficient memory
*/
    }

    se_off(peripherals);
}
