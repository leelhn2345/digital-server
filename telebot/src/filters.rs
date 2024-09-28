use teloxide::types::{Me, Message};

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
pub fn i_got_added(msg: Message, me: Me) -> bool {
    let new_user = msg.new_chat_members();
    let Some(user) = new_user else { return false };

    if user[0].id == me.id {
        tracing::debug!("i got added");
        true
    } else {
        false
    }
}

#[tracing::instrument(skip_all)]
pub fn i_got_removed(msg: Message, me: Me) -> bool {
    let old_user = msg.left_chat_member();
    let Some(user) = old_user else { return false };

    if user.id == me.id {
        tracing::debug!("i got removed");
        true
    } else {
        false
    }
}

#[tracing::instrument(skip_all)]
pub fn group_title_change(msg: Message) -> bool {
    msg.new_chat_title().is_some()
}
