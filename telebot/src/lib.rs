use async_openai::{config::OpenAIConfig, Client};
use filters::{i_got_added, i_got_removed};
use settings::{openai::OpenAISettings, stickers::Stickers};
use sqlx::PgPool;
use teloxide::{
    dispatching::MessageFilterExt,
    requests::Requester,
    types::{Me, Message},
    update_listeners::webhooks,
    utils::command::BotCommands,
};

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
mod filters;
mod handlers;
mod sticker;

#[derive(Clone)]
pub struct BotState {
    #[expect(unused)]
    pool: PgPool,
    #[expect(unused)]
    openai: OpenAISettings,
    #[expect(unused)]
    chatgpt: Client<OpenAIConfig>,
    bot_me: Me,
    stickers: Stickers,
}

impl BotState {
    #[must_use]
    pub async fn new(pool: PgPool, openai: OpenAISettings, bot: &Bot, stickers: Stickers) -> Self {
        let chatgpt = Client::new();

        bot.set_my_commands(commands::Command::bot_commands())
            .await
            .expect("error setting bot commands");

        let me = bot.get_me().await.expect("cannot get details about bot.");

        Self {
            pool,
            openai,
            chatgpt,
            bot_me: me,
            stickers,
        }
    }
}

pub async fn init_bot<T>(bot: Bot, listener: T, app_state: BotState)
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
            Update::filter_message()
                .branch(
                    dptree::entry()
                        .filter_command::<Command>()
                        .endpoint(Command::answer),
                )
                .branch(Message::filter_group_chat_created().endpoint(handlers::me_join))
                .branch(
                    Message::filter_new_chat_members()
                        .branch(dptree::filter(i_got_added).endpoint(handlers::me_join))
                        .branch(dptree::endpoint(handlers::member_join)),
                )
                .branch(
                    Message::filter_left_chat_member()
                        .branch(dptree::filter(i_got_removed).endpoint(handlers::me_leave))
                        .branch(dptree::endpoint(handlers::member_leave)),
                ),
        )
}
