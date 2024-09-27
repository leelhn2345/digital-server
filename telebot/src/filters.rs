use teloxide::types::Message;

use crate::BotState;

#[tracing::instrument(skip_all)]
pub fn is_group_chat(msg: Message) -> bool {
    if msg.chat.is_private() || msg.chat.is_channel() {
        return false;
    }
    true
}

#[tracing::instrument(skip_all)]
pub fn is_not_group_chat(msg: Message) -> bool {
    !is_group_chat(msg)
}

#[tracing::instrument(skip_all)]
pub fn i_got_added(msg: Message, state: BotState) -> bool {
    let new_user = msg.new_chat_members();
    let Some(user) = new_user else { return false };

    if user[0].id == state.bot_me.id {
        tracing::debug!("i got added");
        true
    } else {
        false
    }
}

#[tracing::instrument(skip_all)]
pub fn i_got_removed(msg: Message, state: BotState) -> bool {
    let old_user = msg.left_chat_member();
    let Some(user) = old_user else { return false };

    if user.id == state.bot_me.id {
        tracing::debug!("i got removed");
        true
    } else {
        false
    }
}
