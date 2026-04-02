
#![allow(dead_code)]
#[allow(unused_imports)]
#[allow(clippy::wildcard_imports)]
use sercli::*;
use crate::SavedResponse;
use crate::UserStat;

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
pub struct Chat {
    pub telegram_id: ID,
    pub name: String,
    pub kind: crate::ChatKind,
}
impl Chat {
    pub fn saved_responses<'a>(&self, pool: &'a sqlx::PgPool) -> CrudRequest<'a, SavedResponse> {
        SavedResponse::get(pool).with(SavedResponse::CHAT_ID, self.telegram_id)
    }

    pub fn user_stats<'a>(&self, pool: &'a sqlx::PgPool) -> CrudRequest<'a, UserStat> {
        UserStat::get(pool).with(UserStat::CHAT_ID, self.telegram_id)
    }
}
