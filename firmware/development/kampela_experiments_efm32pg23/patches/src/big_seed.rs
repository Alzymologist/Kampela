//! `substrate-bip39` functionality for 64-byte seed generation from entropy.

use alloc::string::String;
use hmac::Hmac;
use pbkdf2::pbkdf2;
use sha2::Sha512;
use zeroize::Zeroize;

use crate::{error::Error, phrase::check_entropy_length};

/// Verbatim from `substrate-bip39`.
pub fn entropy_to_big_seed(entropy: &[u8], password: &str) -> Result<[u8; 64], Error> {
    check_entropy_length(entropy)?;

    let mut salt = String::with_capacity(8 + password.len());
    salt.push_str("mnemonic");
    salt.push_str(password);

    let mut seed = [0u8; 64];

    pbkdf2::<Hmac<Sha512>>(entropy, salt.as_bytes(), 2048, &mut seed);

    salt.zeroize();

    Ok(seed)
}
