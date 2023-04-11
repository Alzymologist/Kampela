/// Initial state of data in the whole app on loading
///
/// Note that state indicating bricked Kampela is not represented here, as it is hadnled on lower
/// level
#[derive(Debug, PartialEq)]
pub struct AppStateInit {
    /// Data received through NFC channel
    pub nfc: NFCState,
    /// Data stored in flash
    pub storage: StorageState,
}

/// Payload received through NFC channel
#[derive(Debug, PartialEq)]
pub enum NFCState {
    Empty,
}

/// State of Kampela on boot
#[derive(Debug, PartialEq)]
pub struct StorageState {
    /// Set to `true` if pin code is set and secret was generated
    pub key_created: bool,
}

/// Implement this to support platform
pub trait DataInit<InputParameters> {
    fn new(param: InputParameters) -> AppStateInit;
}

/*
pub struct DataState {
    pub 
}*/
