mod app;
mod database;
mod s3;

use app::AppSettings;
use database::DatabaseSettings;
use figment::{
    providers::{Env, Format, Yaml},
    Figment,
};
use s3::S3;
use serde::Deserialize;

use crate::environment::Environment;

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub app: AppSettings,
    pub s3: S3,
    pub database: DatabaseSettings,
}

impl Settings {
    pub fn new(env: &Environment) -> Result<Settings, figment::Error> {
        let base_path =
            std::env::current_dir().expect("failed to determine current working directory");
        let config_dir = base_path.join("settings");

        let env_filename = format!("{}.yaml", env.as_str());

        Figment::new()
            .merge(Yaml::file(config_dir.join("base.yaml")))
            .merge(Yaml::file(config_dir.join(env_filename)))
            .merge(Yaml::file(config_dir.join("stickers.yaml")))
            .merge(Env::prefixed("APP_").split("__"))
            .extract()
    }
}
