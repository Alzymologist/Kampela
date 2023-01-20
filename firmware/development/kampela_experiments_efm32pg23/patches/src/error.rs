//! Errors related to seeds and entropy.

#[derive(Debug)]
pub enum Error {
    InvalidChecksum,
    InvalidEntropy,
    NoWord,
    WordsNumber,
}
