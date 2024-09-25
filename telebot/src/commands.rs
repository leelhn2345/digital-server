use teloxide::{macros::BotCommands, requests::Requester, types::Message, Bot};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(hide)]
    Start,

    #[command(hide)]
    Hello,
}
impl Command {
    pub async fn answer(bot: Bot, msg: Message, cmd: Command) -> anyhow::Result<()> {
        let chat_id = msg.chat.id;

        match cmd {
            Command::Start | Command::Hello => {
                bot.send_message(chat_id, "wtf im not ready yet").await?
            }
        };

        Ok(())
    }
}
