use secrecy::SecretString;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppSettings {
    pub port: u16,
    pub host: String,
    pub cors_allow_origin: SecretString,
    pub public_url: String,
}
