use async_openai::{config::OpenAIConfig, Client};
use handlers::{bot_handler, chat::ChatState};
use settings::{openai::OpenAISettings, stickers::Stickers};
use sqlx::PgPool;
use teloxide::{
    dispatching::dialogue::InMemStorage, requests::Requester, update_listeners::webhooks,
    utils::command::BotCommands,
};

use std::{convert::Infallible, net::SocketAddr};

use teloxide::{
    dispatching::Dispatcher,
    dptree::{self},
    error_handlers::LoggingErrorHandler,
    update_listeners::UpdateListener,
    Bot,
};

mod commands;
mod filters;
mod handlers;
mod sticker;

#[derive(Clone)]
pub struct BotState {
    pool: PgPool,
    openai: OpenAISettings,
    chatgpt: Client<OpenAIConfig>,
    stickers: Stickers,
}

impl BotState {
    #[must_use]
    pub fn new(pool: PgPool, openai: OpenAISettings, stickers: Stickers) -> Self {
        let chatgpt = Client::new();

        Self {
            pool,
            openai,
            chatgpt,
            stickers,
        }
    }
}

pub async fn init_bot<T>(bot: Bot, listener: T, app_state: BotState)
where
    T: UpdateListener<Err = Infallible>,
{
    bot.set_my_commands(commands::Command::bot_commands())
        .await
        .expect("error setting bot commands");

    Box::pin(
        Dispatcher::builder(bot, bot_handler())
            .dependencies(dptree::deps![app_state, InMemStorage::<ChatState>::new()])
            .enable_ctrlc_handler()
            .build()
            .dispatch_with_listener(listener, LoggingErrorHandler::new()),
    )
    .await;
}

pub fn webhook_options(addr: SocketAddr, public_url: &str) -> webhooks::Options {
    let webhook_url = format!("{public_url}/webhook")
        .parse()
        .expect("unable to parse into webhook url");

    webhooks::Options::new(addr, webhook_url)
}
