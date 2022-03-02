use candid::CandidType;
use serde::Deserialize;
use types::CanisterWasm;

#[derive(CandidType, Deserialize, Debug)]
pub struct Args {
    pub user_index_canister_wasm: CanisterWasm,
}

#[derive(CandidType, Deserialize, Debug)]
pub enum Response {
    Success,
    InternalError(String),
}