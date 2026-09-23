use std::str::FromStr;

use serde::Deserialize;

#[derive(PartialEq, Debug)]
pub enum Environment {
    Development,
    Production,
    Staging,
}

impl<'de> Deserialize<'de> for Environment {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl Environment {
    pub fn new() -> Environment {
        std::env::var("APP_ENVIRONMENT")
            .unwrap_or("dev".into())
            .parse()
            .expect("failed to determine environment")
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Staging => "staging",
            Environment::Production => "production",
        }
    }
}

impl FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "dev" | "development" => Ok(Self::Development),
            "stage" | "staging" => Ok(Self::Staging),
            "prod" | "production" => Ok(Self::Production),
            unknown => Err(format!("{unknown} is not a supported environment.")),
        }
    }
}
