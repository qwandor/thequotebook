use sqlx::{postgres::PgRow, FromRow, Row};

pub struct Flash {
    pub notice: Option<String>,
    pub error: Option<String>,
}

#[derive(Clone, Debug, FromRow)]
pub struct Context {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub quote_count: u32,
}

#[derive(Clone, Debug, FromRow)]
pub struct Quote {
    pub id: i32,
    pub quote_text: String,
    pub context_id: i32,
    pub quoter_id: i32,
    pub quotee_id: i32,
    pub hidden: bool,
}

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
                quote_count: 0,
            },
            comments_count: row.try_get("comments_count")?,
        })
    }
}
