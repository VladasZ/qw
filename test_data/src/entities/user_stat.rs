
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
pub struct UserStat {
    pub user_id: ID,
    pub chat_id: ID,
    pub messages: i32,
    pub kto: i32,
    pub llm: i32,
    pub commands: i32,
}