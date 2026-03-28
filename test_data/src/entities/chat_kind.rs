
#[derive(strum::Display, strum::EnumString, serde::Serialize, serde::Deserialize, sqlx::Type, Copy, Clone, Default, PartialEq, Debug)]
#[sqlx(type_name = "chat_kind", rename_all = "lowercase")]
pub enum ChatKind {
    #[default]
    Public,
    Private,
}

impl sercli::reflected::ToReflectedVal<ChatKind> for &str {
    fn to_reflected_val(&self) -> Result<ChatKind, String> {
        use std::str::FromStr;
        Ok(ChatKind::from_str(self).unwrap())
    }
}
