/// A packet used to initiate a key exchange
/// This packet corresponds to protocol message number [SSH_MSG_KEXINIT](crate::messages::SSH_MSG_KEXINIT)
pub struct InitiateKeyExchangePacket {
    /// this field must be [crate::messages::SSH_MSG_KEXINIT](SSH_MSG_KEXINIT)
    message_number: u8,

    /// random bytes
    cookie: [u8; 16],
}
