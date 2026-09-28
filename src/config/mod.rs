use figment::{
    Figment,
    providers::{Format, Yaml},
};
use serde::Deserialize;

use crate::config::{database::Database, environment::Environment};

mod database;
pub mod environment;

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "Environment::new")]
    environment: Environment,

    database: Database,

    log_targets: Vec<String>,
}

impl Config {
    pub fn new() -> Config {
        let base_path =
            std::env::current_dir().expect("failed to determine current working directory");

        let config_dir = base_path.join("configs");

        Figment::new()
            .merge(Yaml::file(config_dir.join("config.yml")))
            .extract()
            .expect("failed to parse application config.")
    }

    pub fn get_environment(&self) -> &Environment {
        &self.environment
    }

    pub fn get_database(&self) -> &Database {
        &self.database
    }

    pub fn get_log_targets(&self) -> &Vec<String> {
        &self.log_targets
    }
}
