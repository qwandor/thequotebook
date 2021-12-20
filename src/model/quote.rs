use super::context::Context;
use super::user::User;
use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Row,
};

#[derive(Clone, Debug, FromRow)]
pub struct Quote {
    pub id: i32,
    pub quote_text: String,
    pub context_id: i32,
    pub quoter_id: i32,
    pub quotee_id: i32,
    pub created_at: DateTime<Utc>,
    pub hidden: bool,
}

#[derive(Clone, Debug)]
pub struct QuoteWithUsers {
    pub quote: Quote,
    pub quoter: User,
    pub quotee: User,
    pub context: Context,
    pub comments_count: i64,
}

impl<'r> FromRow<'r, PgRow> for QuoteWithUsers {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(QuoteWithUsers {
            quote: Quote::from_row(row)?,
            quoter: User {
                id: row.try_get("quoter_id")?,
                email_address: row.try_get("quoter_email_address")?,
                username: row.try_get("quoter_username")?,
                fullname: row.try_get("quoter_fullname")?,
                openid: row.try_get("quoter_openid")?,
            },
            quotee: User {
                id: row.try_get("quotee_id")?,
                email_address: row.try_get("quotee_email_address")?,
                username: row.try_get("quotee_username")?,
                fullname: row.try_get("quotee_fullname")?,
                openid: row.try_get("quotee_openid")?,
            },
            context: Context {
                id: row.try_get("context_id")?,
                name: row.try_get("context_name")?,
                description: row.try_get("context_description")?,
                quotes_count: 0,
            },
            comments_count: row.try_get("comments_count")?,
        })
    }
}
