pub mod app;
pub mod database;
pub mod environment;
pub mod openai;
pub mod redis;
pub mod s3;
pub mod stickers;
pub mod telemetry;

use app::AppSettings;
use database::DatabaseSettings;
use environment::Environment;
use figment::{
    providers::{Env, Format, Yaml},
    Figment,
};
use openai::OpenAISettings;
use redis::RedisSettings;
use s3::S3Settings;
use serde::Deserialize;
use stickers::Stickers;

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub app: AppSettings,
    pub s3: S3Settings,
    pub database: DatabaseSettings,
    pub openai: OpenAISettings,
    pub stickers: Stickers,
    pub redis: RedisSettings,
}

impl Settings {
    pub fn new(env: &Environment) -> Result<Settings, figment::Error> {
        let base_path =
            std::env::current_dir().expect("failed to determine current working directory");
        let config_dir = base_path.join("settings/config");

        let env_filename = format!("{}.yaml", env.as_str());

        Figment::new()
            .merge(Yaml::file(config_dir.join("base.yaml")))
            .merge(Yaml::file(config_dir.join(env_filename)))
            .merge(Yaml::file(config_dir.join("stickers.yaml")))
            .merge(Env::prefixed("APP_").split("__"))
            .extract()
    }
}
