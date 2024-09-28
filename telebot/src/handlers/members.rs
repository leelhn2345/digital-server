use teloxide::{
    payloads::SendMessageSetters,
    requests::Requester,
    types::{Me, Message, ReplyParameters, User},
    Bot,
};

use crate::{handlers::chatroom::delete_chatroom, sticker::send_sticker, BotState};

use super::chatroom::save_chatroom;

pub async fn me_join(bot: Bot, msg: Message, me: Me, state: BotState) -> anyhow::Result<()> {
    let bot_name = me.full_name();
    save_chatroom(&msg, &state.pool).await?;
    let greet = format!("Hello everyone!! I am {bot_name}!");
    send_sticker(&bot, &msg.chat.id, state.stickers.hello).await?;
    bot.send_message(msg.chat.id, greet).await?;
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn me_leave(state: BotState, msg: Message) -> anyhow::Result<()> {
    tracing::debug!("i just left the chat");
    delete_chatroom(&state.pool, msg.chat.id.0).await?;
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

#[tracing::instrument(skip_all)]
pub async fn member_leave(msg: Message, state: BotState, bot: Bot) -> anyhow::Result<()> {
    let Some(member) = msg.left_chat_member() else {
        return Ok(());
    };

    let text = format!("Sayanora {} ~~ 😭😭😭", member.full_name());
    send_sticker(&bot, &msg.chat.id, state.stickers.sad).await?;
    bot.send_message(msg.chat.id, text)
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;

    Ok(())
}
