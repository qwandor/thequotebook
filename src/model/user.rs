use sqlx::{FromRow, Pool, Postgres};

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

    /// Fetches the user with the given ID, if they exist.
    pub async fn fetch_one(pool: &Pool<Postgres>, user_id: i32) -> sqlx::Result<Self> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_one(pool)
            .await
    }
}
