use crate::arch::NameList;
use bincode::{Decode, Encode};

/// A packet used to initiate a key exchange and negotiate the algorithm
///
/// This packet corresponds to protocol message number [SSH_MSG_KEXINIT](crate::messages::SSH_MSG_KEXINIT)
#[derive(Encode, Decode)]
pub struct InitiateKeyExchangePacket {
    /// the message type field
    ///
    /// this field must be [crate::messages::SSH_MSG_KEXINIT](SSH_MSG_KEXINIT)
    message_number: u8,

    /// random bytes
    cookie: [u8; 16],

    // algorithm fields -
    kex_algorithms: NameList,
    server_host_key_algorithms: NameList,
    encryption_algorithms_client_to_server: NameList,
    encryption_algorithms_server_to_client: NameList,
    mac_algorithms_client_to_server: NameList,
    mac_algorithms_server_to_client: NameList,
    compression_algorithms_client_to_server: NameList,
    compression_algorithms_server_to_client: NameList,
    // - algorithm fields
    languages_client_to_server: NameList,
    languages_server_to_client: NameList,

    first_kex_packet_follows: bool,

    /// This field is reserved future use
    _reserved_field: u32,
}