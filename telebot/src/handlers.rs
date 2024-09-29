use chat::{chat_with_users, ChatState};
use chatroom::update_title;
use members::{me_join, me_leave, member_join, member_leave};
use teloxide::{
    dispatching::{
        dialogue::ErasedStorage, DpHandlerDescription, HandlerExt, MessageFilterExt,
        UpdateFilterExt,
    },
    dptree::{self, Handler},
    prelude::DependencyMap,
    types::{Message, Update},
};

use crate::{
    commands::Command,
    filters::{group_title_change, i_got_added, i_got_removed, is_not_group_chat},
};

pub mod chat;
pub mod chatroom;
pub mod members;

pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub fn bot_handler() -> Handler<'static, DependencyMap, HandlerResult, DpHandlerDescription> {
    dptree::entry()
        .inspect(|u: Update| tracing::debug!("{:#?}", u))
        .branch(
            Update::filter_message()
                .enter_dialogue::<Message, ErasedStorage<ChatState>, ChatState>()
                .branch(teloxide::filter_command::<Command, _>().endpoint(Command::answer))
                .branch(Message::filter_group_chat_created().endpoint(me_join))
                .branch(
                    Message::filter_new_chat_members()
                        .branch(dptree::filter(i_got_added).endpoint(me_join))
                        .branch(dptree::endpoint(member_join)),
                )
                .branch(
                    Message::filter_left_chat_member()
                        .branch(dptree::filter(i_got_removed).endpoint(me_leave))
                        .branch(dptree::endpoint(member_leave)),
                )
                .branch(dptree::filter(group_title_change).endpoint(update_title))
                .branch(dptree::filter(is_not_group_chat).endpoint(chat_with_users))
                .branch(dptree::case![ChatState::Talk].endpoint(chat_with_users)),
        )
}
