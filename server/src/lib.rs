use std::{future::Future, net::SocketAddr};

use aws_sdk_s3::Client;
use axum::Router;
use routes::app_router;
use settings::{environment::Environment, telemetry::init_tracing, Settings};
use sqlx::PgPool;
use telebot::{init_bot, BotState};
use teloxide::{
    stop::StopToken,
    update_listeners::{webhooks, UpdateListener},
    Bot,
};

pub mod errors;
mod routes;

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

    init_tracing(&env, vec!["server", "telebot"]);

    tracing::debug!("{settings:#?}");

    let addr = format!("{}:{}", settings.app.host, settings.app.port)
        .parse()
        .expect("unable to get address url");

    let options = telebot::webhook_options(addr, &settings.app.public_url);

    let bot = Bot::from_env();
    let (mut update_listener, stop_flag, teloxide_router) =
        webhooks::axum_to_router(bot.clone(), options)
            .await
            .expect("couldn't setup teloxide webhook");

    let stop_token = update_listener.stop_token();

    let s3 = settings.s3.new_client();
    let pool = settings.database.get_connection_pool().await;

    let app_state = AppState::new(pool.clone(), s3);
    let bot_state = BotState::new(pool, settings.openai, &bot, settings.stickers).await;

    let router = app_router(&env, app_state, settings.app.cors_allow_origin);
    let router = router.merge(teloxide_router);

    init_server(addr, router, stop_token, stop_flag);

    init_bot(bot, update_listener, bot_state).await;
}

fn init_server<T>(addr: SocketAddr, router: Router, stop_token: StopToken, stop_flag: T)
where
    T: Future<Output = ()> + Send + 'static,
{
    tokio::spawn(async move {
        let tcp_listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|_| {
                stop_token.stop();
            })
            .expect("Couldn't bind to the address");

        axum::serve(tcp_listener, router)
            .with_graceful_shutdown(stop_flag)
            .await
            .map_err(|_| {
                stop_token.stop();
            })
            .expect("Axum server error");
    });
}
