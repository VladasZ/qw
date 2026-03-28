
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
pub struct User {
    pub telegram_id: crate::Bigserial,
    pub is_bot: bool,
    pub first_name: String,
    pub username: Option<String>,
    pub nickname: Option<String>,
}
