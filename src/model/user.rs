use sqlx::FromRow;

#[derive(Clone, Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub email_address: Option<String>,
    pub username: Option<String>,
    pub fullname: String,
    pub openid: Option<String>,
}

impl User {
    pub fn username_or_fullname(&self) -> &str {
        self.username.as_deref().unwrap_or(&self.fullname)
    }
}
