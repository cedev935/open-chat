use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{Message, MessageId, MessageIndex, UserId};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub user_id: UserId,
    pub thread_root_message_index: Option<MessageIndex>,
    pub message_ids: Vec<MessageId>,
    pub correlation_id: u64,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub enum Response {
    Success(SuccessResult),
    ChatNotFound,
    UserSuspended,
}

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct SuccessResult {
    pub messages: Vec<Message>,
}
