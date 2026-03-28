
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
pub struct Rooster {
    pub id: crate::Bigserial,
    pub chat_id: i64,
    pub name: String,
    pub sopog: String,
    pub strength: i64,
}
