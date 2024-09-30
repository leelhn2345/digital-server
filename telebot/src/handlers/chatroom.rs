use sqlx::{postgres::PgDatabaseError, PgPool};
use teloxide::types::Message;
use time::OffsetDateTime;

use crate::filters::is_group_chat;

use super::HandlerResult;

pub async fn update_title() -> HandlerResult {
    Ok(())
}

pub async fn delete_chatroom(pool: &PgPool, chat_id: i64) -> Result<(), sqlx::Error> {
    sqlx::query!("delete from tele_chatrooms where id = $1", chat_id)
        .execute(pool)
        .await?;
    Ok(())
}

#[tracing::instrument(skip_all)]
pub async fn save_chatroom(msg: &Message, pool: &PgPool) -> Result<(), sqlx::Error> {
    let id = msg.chat.id.0;
    let title = msg.chat.title();
    let is_group = is_group_chat(msg.to_owned());
    let now = OffsetDateTime::now_utc();

    match sqlx::query!(
        "insert into tele_chatrooms 
        (id, title, is_group, joined_at)
        values 
        ($1, $2, $3, $4)",
        id,
        title,
        is_group,
        now
    )
    .execute(pool)
    .await
    {
        Ok(_) => {
            tracing::debug!("chatroom save");
            Ok(())
        }
        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if let Some(pg_err) = db_err.try_downcast_ref::<PgDatabaseError>() {
                    if pg_err.code() == "23505" {
                        // 23505 is the PostgreSQL error code for unique violation (duplicate key)
                        return Ok(());
                    }
                }
            }
            Err(e)
        }
    }
}
