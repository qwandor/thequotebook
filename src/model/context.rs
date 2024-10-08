use crate::errors::InternalError;
use sqlx::{FromRow, Pool, Postgres};

#[derive(Clone, Debug, FromRow)]
pub struct Context {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub quotes_count: i64,
}

impl Context {
    /// Fetches the context with the given ID, if it exists.
    pub async fn fetch_one(pool: &Pool<Postgres>, context_id: i32) -> Result<Self, InternalError> {
        sqlx::query_as::<_, Self>(
            "SELECT contexts.*,
               (SELECT COUNT(*) FROM quotes WHERE quotes.context_id = contexts.id) as quotes_count
            FROM contexts WHERE id = $1",
        )
        .bind(context_id)
        .fetch_optional(pool)
        .await?
        .ok_or(InternalError::NotFound)
    }

    /// Fetches the top 5 contexts with the most quotes.
    pub async fn fetch_top_5(pool: &Pool<Postgres>) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>(
            "SELECT contexts.*,
               (SELECT COUNT(*) FROM quotes WHERE quotes.context_id = contexts.id) AS quotes_count
             FROM contexts
             ORDER BY quotes_count DESC LIMIT 5",
        )
        .fetch_all(pool)
        .await
    }

    /// Fetches all contexts, starting with the most recently created.
    pub async fn fetch_all(pool: &Pool<Postgres>) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>(
            "SELECT contexts.*,
               (SELECT COUNT(*) FROM quotes WHERE quotes.context_id = contexts.id) AS quotes_count
             FROM contexts
             ORDER BY contexts.created_at DESC",
        )
        .fetch_all(pool)
        .await
    }

    /// Fetches all contexts of which the given user is a member.
    pub async fn fetch_for_user(pool: &Pool<Postgres>, user_id: i32) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>(
            "SELECT contexts.*,
               (SELECT COUNT(*) FROM quotes WHERE quotes.context_id = contexts.id) AS quotes_count
             FROM contexts
               INNER JOIN contexts_users ON context_id = contexts.id
             WHERE user_id = $1
             ORDER BY contexts.created_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }
}
