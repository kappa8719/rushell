use crate::configuration::Configuration;
use bon::bon;
use protocol::{Exchange, ExchangeCause, ExchangeState};
use std::sync::Arc;

pub struct ClientExchange {
    configuration: Arc<Configuration>,
    exchange: Exchange,
    state: ExchangeState,
    cause: ExchangeCause,
}

#[bon]
impl ClientExchange {
    #[builder]
    pub fn new(
        configuration: Arc<Configuration>,
        server_id: String,
        cause: ExchangeCause,
    ) -> ClientExchange {
        Self {
            configuration: configuration.clone(),
            exchange: Exchange::builder()
                .client_id(configuration.client_id.clone())
                .server_id(server_id)
                .build(),
            state: ExchangeState::Created,
            cause,
        }
    }
}
