use teloxide::{
    requests::Requester,
    types::{ChatId, InputFile},
    Bot,
};

use crate::handlers::HandlerResult;

pub async fn send_sticker(
    bot: &Bot,
    chat_id: &ChatId,
    sticker_id: impl Into<String>,
) -> HandlerResult {
    bot.send_sticker(*chat_id, InputFile::file_id(sticker_id))
        .await?;
    Ok(())
}
