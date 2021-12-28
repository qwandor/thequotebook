use super::context::Context;
use super::user::User;
use crate::errors::InternalError;
use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Pool, Postgres, Row,
};

#[derive(Clone, Debug, FromRow)]
pub struct Comment {
    pub id: i32,
    pub quote_id: i32,
    pub user_id: i32,
    pub body: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct CommentWithQuote {
    pub comment: Comment,
    pub quote_text: String,
    pub user: User,
    pub context: Context,
}

impl CommentWithQuote {
    /// Fetches the comment with the given ID, if it exists and is for the given quote.
    pub async fn fetch_one(
        pool: &Pool<Postgres>,
        quote_id: i32,
        comment_id: i32,
    ) -> Result<Self, InternalError> {
        sqlx::query_as::<_, CommentWithQuote>(
            "SELECT comments.*,
               comments.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.quote_text,
               quotes.context_id,
               users.email_address AS user_email_address,
               users.username AS user_username,
               users.fullname AS user_fullname,
               users.openid AS user_openid,
               contexts.name AS context_name,
               contexts.description AS context_description
             FROM comments
               INNER JOIN quotes ON quotes.id = comments.quote_id
               INNER JOIN users ON users.id = comments.user_id
               INNER JOIN contexts ON contexts.id = quotes.context_id
             WHERE comments.quote_id = $1
               AND comments.id = $2",
        )
        .bind(quote_id)
        .bind(comment_id)
        .fetch_optional(pool)
        .await?
        .ok_or(InternalError::NotFound)
    }

    /// Fetches all comments for the given quote.
    pub async fn fetch_all_for_quote(
        pool: &Pool<Postgres>,
        quote_id: i32,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, CommentWithQuote>(
            "SELECT comments.*,
               comments.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.quote_text,
               quotes.context_id,
               users.email_address AS user_email_address,
               users.username AS user_username,
               users.fullname AS user_fullname,
               users.openid AS user_openid,
               contexts.name AS context_name,
               contexts.description AS context_description
             FROM comments
               INNER JOIN quotes ON quotes.id = comments.quote_id
               INNER JOIN users ON users.id = comments.user_id
               INNER JOIN contexts ON contexts.id = quotes.context_id
             WHERE comments.quote_id = $1
             ORDER BY comments.created_at ASC",
        )
        .bind(quote_id)
        .fetch_all(pool)
        .await
    }

    /// Fetches the 5 most recent comments made by the given user.
    pub async fn fetch_5_for_user(pool: &Pool<Postgres>, user_id: i32) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, CommentWithQuote>(
            "SELECT comments.*,
               comments.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.quote_text,
               quotes.context_id,
               users.email_address AS user_email_address,
               users.username AS user_username,
               users.fullname AS user_fullname,
               users.openid AS user_openid,
               contexts.name AS context_name,
               contexts.description AS context_description
             FROM comments
               INNER JOIN quotes ON quotes.id = comments.quote_id
               INNER JOIN users ON users.id = comments.user_id
               INNER JOIN contexts ON contexts.id = quotes.context_id
             WHERE comments.user_id = $1
             ORDER BY comments.created_at DESC
             LIMIT 5",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// Fetches the 5 most recent comments on quotes in the given context.
    pub async fn fetch_5_for_context(
        pool: &Pool<Postgres>,
        context_id: i32,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, CommentWithQuote>(
            "SELECT comments.*,
               comments.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.quote_text,
               quotes.context_id,
               users.email_address AS user_email_address,
               users.username AS user_username,
               users.fullname AS user_fullname,
               users.openid AS user_openid,
               contexts.name AS context_name,
               contexts.description AS context_description
             FROM comments
               INNER JOIN quotes ON quotes.id = comments.quote_id
               INNER JOIN users ON users.id = comments.user_id
               INNER JOIN contexts ON contexts.id = quotes.context_id
             WHERE quotes.context_id = $1
             ORDER BY comments.created_at DESC
             LIMIT 5",
        )
        .bind(context_id)
        .fetch_all(pool)
        .await
    }

    /// Fetches the 5 most recent comments on quotes in contexts of which the given user is a
    /// member.
    pub async fn fetch_5_for_user_contexts(
        pool: &Pool<Postgres>,
        user_id: i32,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, CommentWithQuote>(
            "SELECT comments.*,
               comments.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.quote_text,
               quotes.context_id,
               users.email_address AS user_email_address,
               users.username AS user_username,
               users.fullname AS user_fullname,
               users.openid AS user_openid,
               contexts.name AS context_name,
               contexts.description AS context_description
             FROM comments
               INNER JOIN quotes ON quotes.id = comments.quote_id
               INNER JOIN users ON users.id = comments.user_id
               INNER JOIN contexts ON contexts.id = quotes.context_id
               INNER JOIN contexts_users ON contexts_users.context_id = quotes.context_id
             WHERE contexts_users.user_id = $1
             ORDER BY comments.created_at DESC
             LIMIT 5",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }
}

impl<'r> FromRow<'r, PgRow> for CommentWithQuote {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(CommentWithQuote {
            comment: Comment::from_row(row)?,
            quote_text: row.try_get("quote_text")?,
            user: User {
                id: row.try_get("user_id")?,
                email_address: row.try_get("user_email_address")?,
                username: row.try_get("user_username")?,
                fullname: row.try_get("user_fullname")?,
                openid: row.try_get("user_openid")?,
            },
            context: Context {
                id: row.try_get("context_id")?,
                name: row.try_get("context_name")?,
                description: row.try_get("context_description")?,
                quotes_count: 0,
            },
        })
    }
}

#[derive(Clone, Debug)]
pub struct CommentWithQuotee {
    pub comment: Comment,
    pub quote_text: String,
    pub user: User,
    pub quotee: User,
}

impl CommentWithQuotee {
    /// Fetches all comments, starting with the most recently added.
    pub async fn fetch_all(pool: &Pool<Postgres>) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>(
            "SELECT comments.*,
               comments.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.quote_text,
               quotes.quotee_id,
               users.email_address AS user_email_address,
               users.username AS user_username,
               users.fullname AS user_fullname,
               users.openid AS user_openid,
               quotee.username AS quotee_username,
               quotee.fullname AS quotee_fullname,
               quotee.email_address AS quotee_email_address,
               quotee.openid AS quotee_openid
             FROM comments
               INNER JOIN quotes ON quotes.id = comments.quote_id
               INNER JOIN users ON users.id = comments.user_id
               INNER JOIN users AS quotee ON quotee.id = quotes.quotee_id
             ORDER BY comments.created_at DESC",
        )
        .fetch_all(pool)
        .await
    }

    /// Fetches all comments on quotes in contexts of which the given user is a member.
    pub async fn fetch_all_for_user_contexts(
        pool: &Pool<Postgres>,
        user_id: i32,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>(
            "SELECT comments.*,
               comments.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.quote_text,
               quotes.quotee_id,
               users.email_address AS user_email_address,
               users.username AS user_username,
               users.fullname AS user_fullname,
               users.openid AS user_openid,
               quotee.username AS quotee_username,
               quotee.fullname AS quotee_fullname,
               quotee.email_address AS quotee_email_address,
               quotee.openid AS quotee_openid
             FROM comments
               INNER JOIN quotes ON quotes.id = comments.quote_id
               INNER JOIN users ON users.id = comments.user_id
               INNER JOIN users AS quotee ON quotee.id = quotes.quotee_id
               INNER JOIN contexts_users ON contexts_users.context_id = quotes.context_id
             WHERE contexts_users.user_id = $1
             ORDER BY comments.created_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }
}

impl<'r> FromRow<'r, PgRow> for CommentWithQuotee {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(CommentWithQuotee {
            comment: Comment::from_row(row)?,
            quote_text: row.try_get("quote_text")?,
            user: User {
                id: row.try_get("user_id")?,
                email_address: row.try_get("user_email_address")?,
                username: row.try_get("user_username")?,
                fullname: row.try_get("user_fullname")?,
                openid: row.try_get("user_openid")?,
            },
            quotee: User {
                id: row.try_get("quotee_id")?,
                email_address: row.try_get("quotee_email_address")?,
                username: row.try_get("quotee_username")?,
                fullname: row.try_get("quotee_fullname")?,
                openid: row.try_get("quotee_openid")?,
            },
        })
    }
}
