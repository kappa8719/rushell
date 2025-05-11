pub struct SessionEncryption {

}

pub enum SessionEncryptionState {
    Authenticated,
    InitializeCompression,
    WaitingAuthServiceRequest { is_sent: bool, is_accepted: bool },
    WaitingAuthRequest,
}