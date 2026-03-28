
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
    pub user_id: i64,
    pub chat_id: i64,
    pub messages: i32,
    pub kto: i32,
    pub llm: i32,
    pub commands: i32,
}
