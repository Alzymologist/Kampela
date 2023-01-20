//! `tiny-bip39` functionality for entropy to seed phrase and seed phrase to
//! entropy transformations.

use alloc::{vec::Vec, string::String};
use bitvec::prelude::{BitSlice, BitVec, Msb0};
use sha2::{Digest, Sha256};

use crate::{error::Error, wordlist::WORDLIST_ENGLISH};

pub struct WordList {
    inner: [&'static str; 2048]
}

pub const fn wordlist_english() -> WordList {
    WordList { inner: WORDLIST_ENGLISH }
}

pub struct WordListElement {
    word: &'static str,
    bits11: Bits11,
}

impl WordListElement {
    pub fn word(&self) -> &'static str {
        self.word
    }
    pub fn bits11(&self) -> Bits11 {
        self.bits11
    }
}

impl WordList {
    pub fn get_word(&self, bits: Bits11) -> &'static str {
        self.inner[bits.bits() as usize]
    }

    pub fn get_words_by_prefix(&self, prefix: &str) -> Vec<WordListElement> {
        let start = self.inner
            .binary_search(&prefix)
            .unwrap_or_else(|idx| idx);
        let count = self.inner[start..].iter()
            .take_while(|word| word.starts_with(prefix))
            .count();

        let mut out: Vec<WordListElement> = Vec::with_capacity(count);
        for idx in start..start + count {
            out.push(WordListElement{
                word: self.inner[idx],
                bits11: Bits11::from(idx as u16)
            })
        }
        out
    }

    pub fn get_bits11(&self, word: &str) -> Result<Bits11, Error> {
        let mut found = None;
        for (i, element) in self.inner.iter().enumerate() {
            if element == &word {
                found = Some(i);
                break;
            }
        }
        match found {
            Some(idx) => Ok(Bits11::from(idx as u16)),
            None => Err(Error::NoWord)
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Bits11(u16);

impl Bits11 {
    fn bits(self) -> u32 {
        self.0 as u32
    }
}

impl From<u16> for Bits11 {
    fn from(val: u16) -> Self {
        Bits11(val)
    }
}

impl From<Bits11> for u16 {
    fn from(val: Bits11) -> Self {
        val.0
    }
}

#[derive(Debug, Copy, Clone)]
pub enum MnemonicType {
    Words12,
    Words15,
    Words18,
    Words21,
    Words24,
}

impl MnemonicType {
    fn from(len: usize) -> Result<Self, Error> {
        match len {
            12 => Ok(Self::Words12),
            15 => Ok(Self::Words15),
            18 => Ok(Self::Words18),
            21 => Ok(Self::Words21),
            24 => Ok(Self::Words24),
            _ => Err(Error::WordsNumber)
        }
    }
    fn checksum_bits(&self) -> u8 {
        match &self {
            Self::Words12 => 4,
            Self::Words15 => 5,
            Self::Words18 => 6,
            Self::Words21 => 7,
            Self::Words24 => 8,
        }
    }
    fn entropy_bits(&self) -> usize {
        match &self {
            Self::Words12 => 128,
            Self::Words15 => 160,
            Self::Words18 => 192,
            Self::Words21 => 224,
            Self::Words24 => 256,
        }
    }
    fn total_bits(&self) -> usize {
        self.entropy_bits() + self.checksum_bits() as usize
    }
}

pub fn check_entropy_length(entropy: &[u8]) -> Result<(), Error> {
    if entropy.len() < 16 || entropy.len() > 32 || entropy.len() % 4 != 0 {
        Err(Error::InvalidEntropy)
    }
    else {Ok(())}
}

fn sha256_first_byte(input: &[u8]) -> u8 {
    Sha256::digest(input)[0]
}

pub fn entropy_to_phrase(entropy: &[u8]) -> Result<String, Error> {
    check_entropy_length(entropy)?;
    let wordlist = wordlist_english();
    let checksum_byte = sha256_first_byte(entropy);
    let mut entropy_bits: BitVec<u8, Msb0> = BitVec::with_capacity((entropy.len()+1)*8);
    entropy_bits.extend_from_bitslice(&BitVec::<u8, Msb0>::from_slice(entropy));
    entropy_bits.extend_from_bitslice(&BitVec::<u8, Msb0>::from_element(checksum_byte));
    
    let words: Vec<&'static str> = entropy_bits
        .chunks_exact(11usize)
        .map(|chunk| {
            let mut bits11: u16 = 0;
            for (i, bit) in chunk.into_iter().enumerate() {
                if *bit {bits11 |= 1 << (10-i)}
            }
            wordlist.get_word(Bits11(bits11))
        })
        .collect();
    Ok(words.join(" "))
}

pub fn phrase_to_entropy(phrase: &str) -> Result<Vec<u8>, Error> {
    let wordlist = wordlist_english();
    
    let words: Vec<&str> = phrase.split(' ').collect();
    let mnemonic_type = MnemonicType::from(words.len())?;
    
    let mut entropy_bits: BitVec<u8, Msb0> = BitVec::with_capacity(mnemonic_type.total_bits());
    
    for word in words {
        let bits11 = wordlist.get_bits11(word)?;
        entropy_bits.extend_from_bitslice(&BitSlice::<u8, Msb0>::from_slice(&(bits11.bits() as u16).to_be_bytes())[5..16])
    }
    
    let mut entropy = entropy_bits.into_vec();
    let entropy_len = mnemonic_type.entropy_bits() / 8;
    
    let actual_checksum = checksum(entropy[entropy_len], mnemonic_type.checksum_bits());
    
    entropy.truncate(entropy_len);
    
    let checksum_byte = sha256_first_byte(&entropy);
    
    let expected_checksum = checksum(checksum_byte, mnemonic_type.checksum_bits());
    
    if actual_checksum != expected_checksum {Err(Error::InvalidChecksum)}
    else {Ok(entropy)}
}

fn checksum(source: u8, bits: u8) -> u8 {
    assert!(bits <= 8);
    source >> (8 - bits)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test data taken from `tiny-bip39`.
    static VECTORS: &[[&str; 2]] = &[
        [
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about",
            "00000000000000000000000000000000",
        ],
        [
            "legal winner thank year wave sausage worth useful legal winner thank yellow",
            "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
        ],
        [
            "letter advice cage absurd amount doctor acoustic avoid letter advice cage above",
            "80808080808080808080808080808080",
        ],
        [
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo wrong",
            "ffffffffffffffffffffffffffffffff",
        ],
        [
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon agent",
            "000000000000000000000000000000000000000000000000",
        ],
        [
            "legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal will",
            "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
        ],
        [
            "letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter always",
            "808080808080808080808080808080808080808080808080",
        ],
        [
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo when",
            "ffffffffffffffffffffffffffffffffffffffffffffffff",
        ],
        [
            "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon art",
            "0000000000000000000000000000000000000000000000000000000000000000",
        ],
        [
            "legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth useful legal winner thank year wave sausage worth title",
            "7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f",
        ],
        [
            "letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic avoid letter advice cage absurd amount doctor acoustic bless",
            "8080808080808080808080808080808080808080808080808080808080808080",
        ],
        [
            "zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo zoo vote",
            "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",
        ],
        [
            "ozone drill grab fiber curtain grace pudding thank cruise elder eight picnic",
            "9e885d952ad362caeb4efe34a8e91bd2",
        ],
        [
            "gravity machine north sort system female filter attitude volume fold club stay feature office ecology stable narrow fog",
            "6610b25967cdcca9d59875f5cb50b0ea75433311869e930b",
        ],
        [
            "hamster diagram private dutch cause delay private meat slide toddler razor book happy fancy gospel tennis maple dilemma loan word shrug inflict delay length",
            "68a79eaca2324873eacc50cb9c6eca8cc68ea5d936f98787c60c7ebc74e6ce7c",
        ],
        [
            "scheme spot photo card baby mountain device kick cradle pact join borrow",
            "c0ba5a8e914111210f2bd131f3d5e08d",
        ],
        [
            "horn tenant knee talent sponsor spell gate clip pulse soap slush warm silver nephew swap uncle crack brave",
            "6d9be1ee6ebd27a258115aad99b7317b9c8d28b6d76431c3",
        ],
        [
            "panda eyebrow bullet gorilla call smoke muffin taste mesh discover soft ostrich alcohol speed nation flash devote level hobby quick inner drive ghost inside",
            "9f6a2878b2520799a44ef18bc7df394e7061a224d2c33cd015b157d746869863",
        ],
        [
            "cat swing flag economy stadium alone churn speed unique patch report train",
            "23db8160a31d3e0dca3688ed941adbf3",
        ],
        [
            "light rule cinnamon wrap drastic word pride squirrel upgrade then income fatal apart sustain crack supply proud access",
            "8197a4a47f0425faeaa69deebc05ca29c0a5b5cc76ceacc0",
        ],
        [
            "all hour make first leader extend hole alien behind guard gospel lava path output census museum junior mass reopen famous sing advance salt reform",
            "066dca1a2bb7e8a1db2832148ce9933eea0f3ac9548d793112d9a95c9407efad",
        ],
        [
            "vessel ladder alter error federal sibling chat ability sun glass valve picture",
            "f30f8c1da665478f49b001d94c5fc452",
        ],
        [
            "scissors invite lock maple supreme raw rapid void congress muscle digital elegant little brisk hair mango congress clump",
            "c10ec20dc3cd9f652c7fac2f1230f7a3c828389a14392f05",
        ],
        [
            "void come effort suffer camp survey warrior heavy shoot primary clutch crush open amazing screen patrol group space point ten exist slush involve unfold",
            "f585c11aec520db57dd353c69554b21a89b20fb0650966fa0a9d6f74fd989d8f",
        ]
    ];
    
    #[test]
    fn test_entropy_to_phrase() {
        for vec in VECTORS {
            let entropy = hex::decode(vec[1]).unwrap();
            assert_eq!(entropy_to_phrase(&entropy).unwrap(), vec[0]);
        }
    }
    
    #[test]
    fn test_phrase_to_entropy() {
        for vec in VECTORS {
            let entropy = hex::decode(vec[1]).unwrap();
            assert_eq!(phrase_to_entropy(vec[0]).unwrap(), entropy);
        }
    }
}
