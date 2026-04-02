
#![allow(dead_code)]
#[allow(unused_imports)]
#[allow(clippy::wildcard_imports)]
use sercli::*;

mod reflected {
    pub use sercli::reflected::*;
}

#[derive(
    Debug,
    Default,
    Clone,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    reflected::Reflected,
    sqlx::FromRow,
)]
pub struct SavedResponse {
    pub user_id: ID,
    pub chat_id: ID,
    pub request: String,
    pub response: String,
}