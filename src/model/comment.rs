use super::context::Context;
use super::user::User;
use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
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
