use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn new() -> Environment {
        std::env::var("APP_ENVIRONMENT")
            .unwrap_or("local".into())
            .parse()
            .expect("failed to parse APP_ENVIRONMENT")
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

impl FromStr for Environment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "local" => Ok(Self::Local),
            "production" => Ok(Self::Production),
            unknown => Err(format!("{unknown} is not a supported environment.")),
        }
    }
}
