use anyhow::Context;
use async_openai::{
    config::OpenAIConfig,
    error::OpenAIError,
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageArgs,
        CreateChatCompletionRequestArgs, Role,
    },
    Client,
};
use settings::openai::OpenAISettings;
use sqlx::{postgres::PgDatabaseError, PgPool, Postgres, Transaction};
use teloxide::{
    dispatching::dialogue::InMemStorage,
    payloads::SendMessageSetters,
    prelude::Dialogue,
    requests::Requester,
    types::{Me, Message, ParseMode, ReplyParameters},
    Bot,
};
use time::OffsetDateTime;

use crate::BotState;

use super::chatroom::save_chatroom;

#[derive(Clone, Default)]
pub enum ChatState {
    #[default]
    ShutUp,
    Talk,
}

// use redis state update
pub type ChatDialogue = Dialogue<ChatState, InMemStorage<ChatState>>;

#[tracing::instrument(skip_all)]
#[allow(deprecated)]
pub async fn chat_with_users(
    msg: Message,
    bot: Bot,
    state: BotState,
    me: Me,
) -> anyhow::Result<()> {
    let user = match msg.from {
        Some(ref user) => {
            if user.is_bot {
                return Ok(());
            }
            user.to_owned()
        }
        None => return Ok(()),
    };

    let Some(text_msg) = msg.text() else {
        return Ok(());
    };

    let chat_req = match &user.username {
        Some(username) => ChatCompletionRequestUserMessageArgs::default()
            .content(text_msg)
            .name(username)
            .build()?
            .into(),
        None => ChatCompletionRequestUserMessageArgs::default()
            .content(text_msg)
            .build()?
            .into(),
    };

    let sys_msg = ChatCompletionRequestSystemMessageArgs::default()
        .content(format!(
            "You are a cute and bubbly turtle and your name is {}.",
            me.user.first_name
        ))
        .build()?
        .into();

    let mut tx = state.pool.begin().await?;

    let mut past_msges =
        get_past_chat_logs(&state.pool, msg.chat.id.0, state.openai.chat.past_log_count)
            .await
            .map_err(|e| {
                tracing::error!("{e:#?}");
                e
            })?;

    match save_chat_log(
        &mut tx,
        msg.chat.id.0,
        Role::User,
        text_msg,
        user.username.as_ref(),
    )
    .await
    {
        Ok(()) => Ok(()),
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if let Some(pg_err) = db_err.try_downcast_ref::<PgDatabaseError>() {
                    if pg_err.code() == "23503" {
                        save_chatroom(&msg, &state.pool).await?;
                        bot.send_message(
                            msg.chat.id,
                            "please try again. this message is not processed",
                        )
                        .reply_parameters(ReplyParameters::new(msg.id))
                        .await?;
                    }
                }
            }
            Err(e)
        }
    }?;

    let mut chat_msges = vec![sys_msg];
    chat_msges.append(&mut past_msges);
    chat_msges.push(chat_req);

    let chat_res = chatgpt_chat(state.chatgpt, state.openai, chat_msges).await?;

    save_chat_log(&mut tx, msg.chat.id.0, Role::Assistant, &chat_res, None).await?;

    tx.commit().await?;

    bot.send_message(msg.chat.id, chat_res)
        .parse_mode(ParseMode::Markdown)
        .reply_parameters(ReplyParameters::new(msg.id))
        .await?;

    Ok(())
}

/// client: chatgpt client
/// settings: openai settings such as temperature, number of tokens, etc...
/// chat_msg: the messages to be sent
#[tracing::instrument(skip_all)]
async fn chatgpt_chat(
    client: Client<OpenAIConfig>,
    settings: OpenAISettings,
    chat_msg: Vec<ChatCompletionRequestMessage>,
) -> anyhow::Result<String> {
    let req = CreateChatCompletionRequestArgs::default()
        .model(settings.chat.model)
        .messages(chat_msg)
        .build()?;

    let res = client.chat().create(req).await?;

    let chat_res = res
        .choices
        .first()
        .context("there is no chat completion choice")?
        .to_owned()
        .message
        .content
        .context("chat completion choice is empty")?;

    Ok(chat_res)
}

#[tracing::instrument(skip_all)]
async fn save_chat_log(
    tx: &mut Transaction<'_, Postgres>,
    chat_id: i64,
    role: Role,
    content: impl Into<String>,
    username: Option<&String>,
) -> Result<(), sqlx::Error> {
    let role_str = role.to_string();
    let now = OffsetDateTime::now_utc();

    sqlx::query!(
        "insert into tele_chatlogs
        (chatroom_id, name, role, content, datetime)
        values
        ($1, $2, $3, $4, $5)",
        chat_id,
        username,
        role_str,
        content.into(),
        now
    )
    .execute(&mut **tx)
    .await?;
    Ok(())
}

#[derive(Debug)]
struct PastChatMsg {
    name: Option<String>,
    role: String,
    content: String,
}

#[tracing::instrument(skip_all)]
async fn get_past_chat_logs(
    pool: &PgPool,
    msg_id: i64,
    past_msg_count: i64,
) -> anyhow::Result<Vec<ChatCompletionRequestMessage>> {
    let past_msges: Vec<PastChatMsg> = sqlx::query_as!(
        PastChatMsg,
        "
        select name, role, content from tele_chatlogs
        where chatroom_id = $1
        and datetime >= current_timestamp - interval '1 hour'
        order by datetime desc
        limit $2
        ",
        msg_id,
        past_msg_count
    )
    .fetch_all(pool)
    .await?;
    let mut past_req_msges: Vec<ChatCompletionRequestMessage> = past_msges
        .into_iter()
        .map(|x| match x.role.to_lowercase().as_str().trim() {
            "user" => Ok(ChatCompletionRequestMessage::User(
                ChatCompletionRequestUserMessageArgs::default()
                    .name(x.name.unwrap_or_default())
                    .content(x.content)
                    .build()?,
            )),
            "assistant" => Ok(ChatCompletionRequestMessage::Assistant(
                ChatCompletionRequestAssistantMessageArgs::default()
                    .content(x.content)
                    .build()?,
            )),
            _ => Err(OpenAIError::InvalidArgument("invalid role".to_string())),
        })
        .filter_map(std::result::Result::ok)
        .collect();
    past_req_msges.reverse();
    Ok(past_req_msges)
}
