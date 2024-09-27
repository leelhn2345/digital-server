use teloxide::{requests::Requester, types::Message, utils::command::BotCommands, Bot};
use time::{format_description::well_known::Rfc2822, macros::offset, OffsetDateTime};

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
}
impl Command {
    #[tracing::instrument(skip_all)]
    pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> anyhow::Result<()> {
        let chat_id = msg.chat.id;

        match cmd {
            Self::Start => bot.send_message(chat_id, "wtf im not ready yet").await?,
            Self::Help => {
                bot.send_message(chat_id, Self::descriptions().to_string())
                    .await?
            }
            Self::DateTime => {
                let now = OffsetDateTime::now_utc()
                    .to_offset(offset!(+8))
                    .format(&Rfc2822)?;
                bot.send_message(chat_id, now).await?
            }
        };

        Ok(())
    }
}
