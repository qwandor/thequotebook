use crate::errors::InternalError;
use sqlx::{FromRow, Pool, Postgres};

#[derive(Clone, Debug, Eq, FromRow, PartialEq)]
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
    pub async fn fetch_one(pool: &Pool<Postgres>, user_id: i32) -> Result<Self, InternalError> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(user_id)
            .fetch_optional(pool)
            .await?
            .ok_or(InternalError::NotFound)
    }

    /// Fetches the user with the given email address, or `None`.
    pub async fn fetch_by_email(
        pool: &Pool<Postgres>,
        email_address: &str,
    ) -> Result<Option<Self>, InternalError> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE email_address = $1")
                .bind(email_address)
                .fetch_optional(pool)
                .await?,
        )
    }

    /// Adds the given user to the given context, if they are not already a member.
    pub async fn join_context(
        pool: &Pool<Postgres>,
        user_id: i32,
        context_id: i32,
    ) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO contexts_users (user_id, context_id)
             VALUES ($1, $2)
             ON CONFLICT DO NOTHING",
        )
        .bind(user_id)
        .bind(context_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    /// Removes the given user from the given context.
    pub async fn leave_context(
        pool: &Pool<Postgres>,
        user_id: i32,
        context_id: i32,
    ) -> sqlx::Result<()> {
        sqlx::query(
            "DELETE FROM contexts_users
             WHERE user_id = $1 AND context_id = $2",
        )
        .bind(user_id)
        .bind(context_id)
        .execute(pool)
        .await?;
        Ok(())
    }
}
