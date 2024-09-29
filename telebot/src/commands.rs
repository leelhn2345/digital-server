use teloxide::{requests::Requester, types::Message, utils::command::BotCommands, Bot};
use time::{format_description::well_known::Rfc2822, macros::offset, OffsetDateTime};

use crate::{
    filters::is_group_chat,
    handlers::{
        chat::{ChatDialogue, ChatState},
        chatroom::save_chatroom,
        HandlerResult,
    },
    sticker::send_sticker,
    BotState,
};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(hide)]
    Start,
    /// See all available commands
    Help,
    /// Current datetime (GMT+8)
    DateTime,
    /// Start responding to messages
    Chat,
    /// Stop responding to messages
    ShutUp,
}
impl Command {
    #[tracing::instrument(skip_all)]
    pub async fn answer(
        bot: Bot,
        msg: Message,
        cmd: Command,
        state: BotState,
        chat_dialogue: ChatDialogue,
    ) -> HandlerResult {
        let chat_id = msg.chat.id;

        match cmd {
            Self::Start => {
                save_chatroom(&msg, &state.pool).await.map_err(|e| {
                    tracing::error!("{e:#?}");
                    e
                })?;
                bot.send_message(chat_id, "wtf im not ready yet").await?;
            }
            Self::Help => {
                bot.send_message(chat_id, Self::descriptions().to_string())
                    .await?;
            }
            Self::DateTime => {
                let now = OffsetDateTime::now_utc()
                    .to_offset(offset!(+8))
                    .format(&Rfc2822)?;
                bot.send_message(chat_id, now).await?;
            }
            Self::Chat => {
                if is_group_chat(msg) {
                    chat_dialogue.update(ChatState::Talk).await?;
                    send_sticker(&bot, &chat_id, state.stickers.hello).await?;
                    bot.send_message(chat_id, "Hello! What do you wanna chat about?? 😊")
                        .await?;
                } else {
                    bot.send_message(chat_id, "This command is only available in group chats.")
                        .await?;
                }
            }
            Self::ShutUp => {
                if is_group_chat(msg) {
                    chat_dialogue.update(ChatState::ShutUp).await?;
                    send_sticker(&bot, &chat_id, state.stickers.whatever).await?;
                    bot.send_message(chat_id, "Huh?! Whatever 🙄. Byebye I'm off.")
                        .await?;
                } else {
                    bot.send_message(chat_id, "This command is only available in group chats.")
                        .await?;
                }
            }
        };

        Ok(())
    }
}
