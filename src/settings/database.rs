use std::time::Duration;

use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use sqlx::ConnectOptions;
use sqlx::{
    migrate::MigrateDatabase,
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};

use crate::environment::Environment;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl DatabaseSettings {
    fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
    }

    fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }

    async fn check_db_exists(&self, url: &str) -> bool {
        sqlx::Postgres::database_exists(url)
            .await
            .expect("can't check if database exists or not")
    }

    async fn create_db(&self, url: &str) {
        sqlx::Postgres::create_database(url)
            .await
            .expect("can't create database");
    }

    pub async fn get_connection_pool(self, env: &Environment) -> PgPool {
        let options = self.with_db();

        let db_url = options.to_url_lossy().to_string();

        if !self.check_db_exists(&db_url).await {
            self.create_db(&db_url).await;
        }

        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(2))
            .connect_lazy_with(options);

        if *env == Environment::Production {
            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .expect("cannot run db migration");
        }

        pool
    }
}
