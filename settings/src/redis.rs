use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct RedisSettings {
    pub host: String,
    pub port: u16,
    pub password: SecretString,
}

impl RedisSettings {
    #[must_use]
    pub fn connection_string(&self) -> String {
        format!(
            "redis://:{}@{}:{}",
            self.password.expose_secret(),
            self.host,
            self.port
        )
    }
}
