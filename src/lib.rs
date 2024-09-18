use aws_sdk_s3::Client;
use environment::Environment;
use routes::app_router;
use settings::Settings;
use sqlx::PgPool;
use telemetry::init_tracing;

mod environment;
pub mod errors;
mod routes;
mod settings;
mod telemetry;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub s3: Client,
}

impl AppState {
    fn new(pool: PgPool, s3: Client) -> Self {
        AppState { pool, s3 }
    }
}

pub async fn init_app() {
    let env = Environment::new();
    let settings = Settings::new(&env).expect("failed to parse settings");

    init_tracing(&env, vec!["digital_server"]);

    let s3 = settings.s3.new_client();
    let pool = settings.database.get_connection_pool(&env).await;

    let app_state = AppState::new(pool, s3);

    let cors_allow_origin = settings.app.cors_allow_origin;

    let router = app_router(&env, app_state, &cors_allow_origin);

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", settings.app.host, settings.app.port))
            .await
            .unwrap();

    axum::serve(listener, router)
        .await
        .expect("can't start server");
}
