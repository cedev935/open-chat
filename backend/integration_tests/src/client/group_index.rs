use crate::{generate_query_call, generate_update_call};
use group_index_canister::*;

// Queries
generate_query_call!(search);

// Updates
generate_update_call!(freeze_group);
generate_update_call!(unfreeze_group);