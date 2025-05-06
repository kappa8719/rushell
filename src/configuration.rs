use bon::Builder;
use std::time::Duration;

#[derive(Builder, Clone)]
pub struct Keepalive {
    #[builder(default)]
    interval: Duration,
    #[builder(default)]
    tries: usize,
}

impl Default for Keepalive {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(60),
            tries: 3,
        }
    }
}

#[derive(Clone, Builder)]
pub struct Configuration {
    pub client_id: String,
    pub inactivity_timeout: Option<Duration>,
    pub keepalive: Option<Keepalive>,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            client_id: format!(
                "SSH-2.0-{}_{}",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ),
            inactivity_timeout: None,
            keepalive: None,
        }
    }
}