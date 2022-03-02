use candid::Principal;
use ic_ledger_types::Tokens;

mod alert;
mod avatar;
mod canister_upgrade_status;
mod canister_wasm;
mod challenge;
mod chat_id;
mod chat_summary;
mod confirmation_code_sms;
mod cryptocurrency;
mod cycles;
mod deleted_group_info;
mod error;
mod event_index;
mod event_wrapper;
mod events;
mod field_too_long;
mod group_activity;
mod group_match;
mod http;
mod indexed_event;
mod mention;
mod message;
mod message_content;
mod message_id;
mod message_index;
mod message_match;
mod notifications;
mod option;
mod participant;
mod phone_number;
mod polls;
mod reactions;
mod registration_fee;
mod role;
mod subscription;
mod timestamped;
mod transaction;
mod user;
mod user_preferences;
mod user_summary;
mod version;

pub use alert::*;
pub use avatar::*;
pub use canister_upgrade_status::*;
pub use canister_wasm::*;
pub use challenge::*;
pub use chat_id::*;
pub use chat_summary::*;
pub use confirmation_code_sms::*;
pub use cryptocurrency::*;
pub use cycles::*;
pub use deleted_group_info::*;
pub use error::*;
pub use event_index::*;
pub use event_wrapper::*;
pub use events::*;
pub use field_too_long::*;
pub use group_activity::*;
pub use group_match::*;
pub use http::*;
pub use indexed_event::*;
pub use mention::*;
pub use message::*;
pub use message_content::*;
pub use message_id::*;
pub use message_index::*;
pub use message_match::*;
pub use notifications::*;
pub use option::*;
pub use participant::*;
pub use phone_number::*;
pub use polls::*;
pub use reactions::*;
pub use registration_fee::*;
pub use role::*;
pub use subscription::*;
pub use timestamped::*;
pub use transaction::*;
pub use user::*;
pub use user_preferences::*;
pub use user_summary::*;
pub use version::*;

pub type CanisterId = Principal;
pub type ICP = Tokens;
pub type Milliseconds = u64;
pub type TimestampMillis = u64;
pub type TimestampNanos = u64;
pub type Salt = [u8; 32];