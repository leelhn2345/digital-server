use figment::{Figment, providers::Env};
use serde::Deserialize;

use crate::config::environment::Environment;

pub mod environment;

#[derive(Deserialize)]
pub struct Config {
    #[serde(default = "Environment::new")]
    environment: Environment,
}

impl Config {
    pub fn new() -> Config {
        Figment::new()
            .merge(Env::prefixed("APP_"))
            .extract()
            .expect("failed to parse application config.")
    }

    pub fn get_environment(&self) -> &Environment {
        &self.environment
    }
}
