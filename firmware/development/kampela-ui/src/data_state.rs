/// Initial state of data in the whole app on loading
///
/// Note that state indicating bricked Kampela is not represented here, as it is hadnled on lower
/// level
pub struct AppStateInit {
    /// Data received through NFC channel
    nfc: NFCState,
    /// Data stored in flash
    storage: StorageState,
}

/// Payload received through NFC channel
pub enum NFCState {
    Empty,
}

/// State of Kampela on boot
pub struct StorageState {
    /// Set to `true` if pin code is set and secret was generated
    key_created: bool,
}

