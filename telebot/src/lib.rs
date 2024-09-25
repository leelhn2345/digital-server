use sqlx::PgPool;
use teloxide::update_listeners::webhooks;

use std::{convert::Infallible, net::SocketAddr};

use commands::Command;
use teloxide::{
    dispatching::{Dispatcher, DpHandlerDescription, HandlerExt, UpdateFilterExt},
    dptree::{self, Handler},
    error_handlers::LoggingErrorHandler,
    prelude::DependencyMap,
    types::Update,
    update_listeners::UpdateListener,
    Bot,
};

mod commands;

pub struct BotAppState {
    #[expect(unused)]
    pool: PgPool,
}

impl BotAppState {
    #[must_use]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

pub async fn init_bot<T>(bot: Bot, listener: T, app_state: BotAppState)
where
    T: UpdateListener<Err = Infallible>,
{
    Box::pin(
        Dispatcher::builder(bot, bot_handler())
            .dependencies(dptree::deps![app_state])
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

fn bot_handler() -> Handler<'static, DependencyMap, anyhow::Result<()>, DpHandlerDescription> {
    dptree::entry()
        .inspect(|u: Update| tracing::debug!("{:#?}", u))
        .branch(
            Update::filter_message().branch(
                dptree::entry()
                    .filter_command::<Command>()
                    .endpoint(Command::answer),
            ),
        )
}
