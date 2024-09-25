use aws_sdk_s3::{
    config::{Credentials, Region},
    Client, Config,
};
use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct S3 {
    region: String,
    access_key: SecretString,
    secret_key: SecretString,
    endpoint_url: String,
}

impl S3 {
    pub fn new_client(self) -> Client {
        let creds = Credentials::new(
            self.access_key.expose_secret(),
            self.secret_key.expose_secret(),
            None,
            None,
            "s3-creds",
        );
        let conf = Config::builder()
            .credentials_provider(creds)
            .endpoint_url(self.endpoint_url)
            .region(Region::new(self.region))
            .force_path_style(true)
            .build();

        Client::from_conf(conf)
    }
}
