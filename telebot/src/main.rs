use settings::{telemetry::init_tracing, Environment, Settings};
use telebot::{init_bot, BotAppState};
use teloxide::{update_listeners::webhooks, Bot};

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();
    let env = Environment::new();

    init_tracing(&env, vec!["telebot"]);

    let settings = Settings::new(&env).expect("can't parse settings");

    let addr = format!("{}:{}", settings.app.host, settings.app.port)
        .parse()
        .expect("unable to get address url");

    let options = telebot::webhook_options(addr, &settings.app.public_url);
    let listener = webhooks::axum(bot.clone(), options)
        .await
        .expect("can't set webhook");

    let pool = settings.database.get_connection_pool().await;

    let app_state = BotAppState::new(pool);

    init_bot(bot, listener, app_state).await;
}
