use bon::{bon, builder};

#[derive(Clone)]
pub struct Exchange {
    pub client_id: String,
    pub server_id: String,
}

#[bon]
impl Exchange {
    #[builder]
    pub fn new(client_id: String, server_id: String) -> Exchange {
        Self {
            client_id,
            server_id,
        }
    }
}

#[derive(Clone, Debug)]
pub enum ExchangeCause {
    Initial,
    ReKey { strict: bool, session_id: String },
}

#[derive(Clone, Debug)]
pub enum ExchangeState {
    Created,
    WaitingForGexReply,
    WaitingForDhReply,
    WaitingForNewKeys,
}
