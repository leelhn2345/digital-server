use teloxide::{
    payloads::SendMessageSetters,
    requests::Requester,
    types::{Message, ReplyParameters, User},
    Bot,
};

use crate::{sticker::send_sticker, BotState};

pub async fn me_join(bot: Bot, msg: Message, state: BotState) -> anyhow::Result<()> {
    let bot_name = &state.bot_me.full_name();
    let greet = format!("Hello everyone!! I am {bot_name}!");
    send_sticker(&bot, &msg.chat.id, state.stickers.hello).await?;
    bot.send_message(msg.chat.id, greet).await?;
    Ok(())
}

/// TODO:
pub async fn me_leave() -> anyhow::Result<()> {
    Ok(())
}

pub async fn member_join(bot: Bot, msg: Message, state: BotState) -> anyhow::Result<()> {
    let Some(new_users) = msg.new_chat_members() else {
        return Ok(());
    };

    let users: Vec<User> = new_users
        .iter()
        .filter(|x| !x.is_bot)
        .map(std::borrow::ToOwned::to_owned)
        .collect();

    if users.is_empty() {
        return Ok(());
    };

    for user in users {
        tokio::spawn({
            let bot = bot.clone();
            async move {
                let text = match user.username {
                    Some(username) => format!("Hello @{username}!"),
                    None => format!("Hello {}!", user.first_name),
                };
                // not interested in result
                let _ = bot
                    .send_message(msg.chat.id, text)
                    .reply_parameters(ReplyParameters::new(msg.id))
                    .await;
            }
        });
    }

    send_sticker(&bot, &msg.chat.id, state.stickers.hello).await?;
    Ok(())
}

pub async fn member_leave() -> anyhow::Result<()> {
    Ok(())
}
