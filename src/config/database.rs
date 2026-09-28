use std::time::Duration;

use secrecy::{ExposeSecret, SecretString};
use serde::Deserialize;
use sqlx::{
    ConnectOptions, PgPool,
    migrate::MigrateDatabase,
    postgres::{PgConnectOptions, PgPoolOptions},
};

const CONNECTION_TIMEOUT: u64 = 2;

#[derive(Deserialize, Debug)]
pub struct Database {
    username: String,
    password: SecretString,
    port: u16,
    host: String,
    name: String,
}

impl Database {
    fn without_db(&self) -> PgConnectOptions {
        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
    }

    fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.name)
    }

    async fn check_db_exists(&self, url: &str) -> bool {
        let db_exist_future = sqlx::Postgres::database_exists(url);

        tokio::time::timeout(Duration::from_secs(CONNECTION_TIMEOUT), db_exist_future)
            .await
            .expect("timed out connecting to postgres")
            .expect("can't check if database exists or not")
    }

    async fn create_db(&self, url: &str) {
        sqlx::Postgres::create_database(url)
            .await
            .expect("can't create database");
    }

    pub async fn get_connection_pool(&self) -> PgPool {
        let options = self.with_db();

        let db_url = options.to_url_lossy().to_string();

        if !self.check_db_exists(&db_url).await {
            self.create_db(&db_url).await;
        }

        let pool = PgPoolOptions::new()
            .acquire_timeout(Duration::from_secs(CONNECTION_TIMEOUT))
            .connect_lazy_with(options);

        // self.migrate(&pool).await;

        pool
    }

    #[allow(unused)]
    async fn migrate(&self, pool: &PgPool) {
        sqlx::migrate!("./migrations")
            .run(pool)
            .await
            .expect("cannot run db migration");
    }
}
