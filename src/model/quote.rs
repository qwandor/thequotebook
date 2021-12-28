use super::context::Context;
use super::user::User;
use crate::errors::InternalError;
use paginate::{Page, Pages};
use sqlx::{
    postgres::PgRow,
    types::chrono::{DateTime, Utc},
    FromRow, Pool, Postgres, Row,
};

#[derive(Clone, Debug, FromRow)]
pub struct Quote {
    pub id: i32,
    pub quote_text: String,
    pub context_id: i32,
    pub quoter_id: i32,
    pub quotee_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub hidden: bool,
}

impl Quote {
    /// Fetches the quote with the given ID, if it exists.
    pub async fn fetch_one(pool: &Pool<Postgres>, quote_id: i32) -> Result<Self, InternalError> {
        sqlx::query_as::<_, Quote>(
            "SELECT quotes.*,
               quotes.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.updated_at AT TIME ZONE 'UTC' AS updated_at
             FROM quotes
             WHERE id = $1",
        )
        .bind(quote_id)
        .fetch_optional(pool)
        .await?
        .ok_or(InternalError::NotFound)
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

impl QuoteWithUsers {
    /// Fetches the quote with the given ID, if it exists.
    pub async fn fetch_one(pool: &Pool<Postgres>, quote_id: i32) -> Result<Self, InternalError> {
        sqlx::query_as::<_, QuoteWithUsers>(
            "SELECT quotes.*,
               quotes.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.updated_at AT TIME ZONE 'UTC' AS updated_at,
               (SELECT COUNT(*) FROM comments WHERE comments.quote_id = quotes.id) AS comments_count,
               quoter.username AS quoter_username,
               quoter.fullname AS quoter_fullname,
               quoter.email_address AS quoter_email_address,
               quoter.openid AS quoter_openid,
               quotee.username AS quotee_username,
               quotee.fullname AS quotee_fullname,
               quotee.email_address AS quotee_email_address,
               quotee.openid AS quotee_openid,
               contexts.name AS context_name,
               contexts.description AS context_description
             FROM quotes
               INNER JOIN users AS quoter ON quoter.id = quoter_id
               INNER JOIN users AS quotee ON quotee.id = quotee_id
               INNER JOIN contexts ON contexts.id = context_id
             WHERE quotes.id = $1",
        )
        .bind(quote_id)
        .fetch_optional(pool)
        .await?
        .ok_or(InternalError::NotFound)
    }

    /// Fetches all quotes.
    pub async fn fetch_all(pool: &Pool<Postgres>) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, QuoteWithUsers>(
            "SELECT quotes.*,
               quotes.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.updated_at AT TIME ZONE 'UTC' AS updated_at,
               (SELECT COUNT(*) FROM comments WHERE comments.quote_id = quotes.id) AS comments_count,
               quoter.username AS quoter_username,
               quoter.fullname AS quoter_fullname,
               quoter.email_address AS quoter_email_address,
               quoter.openid AS quoter_openid,
               quotee.username AS quotee_username,
               quotee.fullname AS quotee_fullname,
               quotee.email_address AS quotee_email_address,
               quotee.openid AS quotee_openid,
               contexts.name AS context_name,
               contexts.description AS context_description
             FROM quotes
               INNER JOIN users AS quoter ON quoter.id = quoter_id
               INNER JOIN users AS quotee ON quotee.id = quotee_id
               INNER JOIN contexts ON contexts.id = context_id
             WHERE NOT hidden
             ORDER BY contexts.created_at DESC",
        )
        .fetch_all(pool)
        .await
    }

    /// Returns the number of non-hidden quotes.
    pub async fn count(pool: &Pool<Postgres>) -> sqlx::Result<usize> {
        Ok(
            sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM quotes WHERE NOT hidden")
                .fetch_one(pool)
                .await? as usize,
        )
    }

    /// Fetches all non-hidden quotes within the given page.
    pub async fn fetch_page(
        pool: &Pool<Postgres>,
        pages: &Pages,
        page: &Page,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>(
            "SELECT quotes.*,
               quotes.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.updated_at AT TIME ZONE 'UTC' AS updated_at,
               (SELECT COUNT(*) FROM comments WHERE comments.quote_id = quotes.id) AS comments_count,
               quoter.username AS quoter_username,
               quoter.fullname AS quoter_fullname,
               quoter.email_address AS quoter_email_address,
               quoter.openid AS quoter_openid,
               quotee.username AS quotee_username,
               quotee.fullname AS quotee_fullname,
               quotee.email_address AS quotee_email_address,
               quotee.openid AS quotee_openid,
               contexts.name AS context_name,
               contexts.description AS context_description
             FROM quotes
               INNER JOIN users AS quoter ON quoter.id = quoter_id
               INNER JOIN users AS quotee ON quotee.id = quotee_id
               INNER JOIN contexts ON contexts.id = context_id
             WHERE NOT hidden
             ORDER BY quotes.created_at DESC
             LIMIT $1 OFFSET $2",
        )
        .bind(pages.limit() as i64)
        .bind(page.start as i64)
        .fetch_all(pool)
        .await
    }

    /// Fetches all non-hidden quotes of the given quotee.
    pub async fn fetch_all_for_quotee(
        pool: &Pool<Postgres>,
        quotee_id: i32,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>(
            "SELECT quotes.*,
               quotes.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.updated_at AT TIME ZONE 'UTC' AS updated_at,
               (SELECT COUNT(*) FROM comments WHERE comments.quote_id = quotes.id) AS comments_count,
               quoter.username AS quoter_username,
               quoter.fullname AS quoter_fullname,
               quoter.email_address AS quoter_email_address,
               quoter.openid AS quoter_openid,
               quotee.username AS quotee_username,
               quotee.fullname AS quotee_fullname,
               quotee.email_address AS quotee_email_address,
               quotee.openid AS quotee_openid,
               contexts.name AS context_name,
               contexts.description AS context_description
             FROM quotes
               INNER JOIN users AS quoter ON quoter.id = quoter_id
               INNER JOIN users AS quotee ON quotee.id = quotee_id
               INNER JOIN contexts ON contexts.id = context_id
             WHERE NOT hidden AND quotes.quotee_id = $1
             ORDER BY quotes.created_at DESC",
            )
            .bind(quotee_id)
            .fetch_all(pool)
            .await
    }

    /// Returns the number of non-hidden quotes of the given quotee.
    pub async fn count_for_quotee(pool: &Pool<Postgres>, quotee_id: i32) -> sqlx::Result<usize> {
        Ok(sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*)
             FROM quotes
             WHERE NOT hidden AND quotes.quotee_id = $1",
        )
        .bind(quotee_id)
        .fetch_one(pool)
        .await? as usize)
    }

    /// Fetches non-hidden quotes of the given quotee, within the given page.
    pub async fn fetch_page_for_quotee(
        pool: &Pool<Postgres>,
        quotee_id: i32,
        pages: &Pages,
        page: &Page,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>(
            "SELECT quotes.*,
               quotes.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.updated_at AT TIME ZONE 'UTC' AS updated_at,
               (SELECT COUNT(*) FROM comments WHERE comments.quote_id = quotes.id) AS comments_count,
               quoter.username AS quoter_username,
               quoter.fullname AS quoter_fullname,
               quoter.email_address AS quoter_email_address,
               quoter.openid AS quoter_openid,
               quotee.username AS quotee_username,
               quotee.fullname AS quotee_fullname,
               quotee.email_address AS quotee_email_address,
               quotee.openid AS quotee_openid,
               contexts.name AS context_name,
               contexts.description AS context_description
             FROM quotes
               INNER JOIN users AS quoter ON quoter.id = quoter_id
               INNER JOIN users AS quotee ON quotee.id = quotee_id
               INNER JOIN contexts ON contexts.id = context_id
             WHERE NOT hidden AND quotes.quotee_id = $1
             ORDER BY quotes.created_at DESC
             LIMIT $2 OFFSET $3",
            )
            .bind(quotee_id)
            .bind(pages.limit() as i64)
            .bind(page.start as i64)
            .fetch_all(pool)
            .await
    }

    /// Fetches all non-hidden quotes in the given context.
    pub async fn fetch_all_for_context(
        pool: &Pool<Postgres>,
        context_id: i32,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>(
            "SELECT quotes.*,
               quotes.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.updated_at AT TIME ZONE 'UTC' AS updated_at,
               (SELECT COUNT(*) FROM comments WHERE comments.quote_id = quotes.id) AS comments_count,
               quoter.username AS quoter_username,
               quoter.fullname AS quoter_fullname,
               quoter.email_address AS quoter_email_address,
               quoter.openid AS quoter_openid,
               quotee.username AS quotee_username,
               quotee.fullname AS quotee_fullname,
               quotee.email_address AS quotee_email_address,
               quotee.openid AS quotee_openid,
               contexts.name AS context_name,
               contexts.description AS context_description
             FROM quotes
               INNER JOIN users AS quoter ON quoter.id = quoter_id
               INNER JOIN users AS quotee ON quotee.id = quotee_id
               INNER JOIN contexts ON contexts.id = context_id
             WHERE NOT hidden AND quotes.context_id = $1
             ORDER BY quotes.created_at DESC",
            )
            .bind(context_id)
            .fetch_all(pool)
            .await
    }

    /// Returns the number of non-hidden quotes in the given context.
    pub async fn count_for_context(pool: &Pool<Postgres>, context_id: i32) -> sqlx::Result<usize> {
        Ok(sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*)
             FROM quotes
             WHERE NOT hidden AND quotes.context_id = $1",
        )
        .bind(context_id)
        .fetch_one(pool)
        .await? as usize)
    }

    /// Fetches non-hidden quotes in the given context, within the given page.
    pub async fn fetch_page_for_context(
        pool: &Pool<Postgres>,
        context_id: i32,
        pages: &Pages,
        page: &Page,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>(
            "SELECT quotes.*,
               quotes.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.updated_at AT TIME ZONE 'UTC' AS updated_at,
               (SELECT COUNT(*) FROM comments WHERE comments.quote_id = quotes.id) AS comments_count,
               quoter.username AS quoter_username,
               quoter.fullname AS quoter_fullname,
               quoter.email_address AS quoter_email_address,
               quoter.openid AS quoter_openid,
               quotee.username AS quotee_username,
               quotee.fullname AS quotee_fullname,
               quotee.email_address AS quotee_email_address,
               quotee.openid AS quotee_openid,
               contexts.name AS context_name,
               contexts.description AS context_description
             FROM quotes
               INNER JOIN users AS quoter ON quoter.id = quoter_id
               INNER JOIN users AS quotee ON quotee.id = quotee_id
               INNER JOIN contexts ON contexts.id = context_id
             WHERE NOT hidden AND quotes.context_id = $1
             ORDER BY quotes.created_at DESC
             LIMIT $2 OFFSET $3",
            )
            .bind(context_id)
            .bind(pages.limit() as i64)
            .bind(page.start as i64)
            .fetch_all(pool)
            .await
    }

    /// Fetches all non-hidden quotes in contexts of which the given user is a member.
    pub async fn fetch_all_for_user_contexts(
        pool: &Pool<Postgres>,
        user_id: i32,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>(
            "SELECT quotes.*,
               quotes.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.updated_at AT TIME ZONE 'UTC' AS updated_at,
               (SELECT COUNT(*) FROM comments WHERE comments.quote_id = quotes.id) AS comments_count,
               quoter.username AS quoter_username,
               quoter.fullname AS quoter_fullname,
               quoter.email_address AS quoter_email_address,
               quoter.openid AS quoter_openid,
               quotee.username AS quotee_username,
               quotee.fullname AS quotee_fullname,
               quotee.email_address AS quotee_email_address,
               quotee.openid AS quotee_openid,
               contexts.name AS context_name,
               contexts.description AS context_description
             FROM quotes
               INNER JOIN users AS quoter ON quoter.id = quoter_id
               INNER JOIN users AS quotee ON quotee.id = quotee_id
               INNER JOIN contexts ON contexts.id = context_id
               INNER JOIN contexts_users ON contexts_users.context_id = quotes.context_id
             WHERE NOT hidden AND contexts_users.user_id = $1
             ORDER BY quotes.created_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    /// Returns the number of non-hidden quotes in contexts of which the given user is a member.
    pub async fn count_for_user_contexts(
        pool: &Pool<Postgres>,
        user_id: i32,
    ) -> sqlx::Result<usize> {
        Ok(sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*)
             FROM quotes
               INNER JOIN contexts_users ON contexts_users.context_id = quotes.context_id
             WHERE NOT hidden AND contexts_users.user_id = $1",
        )
        .bind(user_id)
        .fetch_one(pool)
        .await? as usize)
    }

    /// Fetches non-hidden quotes in contexts of which the given user is a member, within the given page.
    pub async fn fetch_page_for_user_contexts(
        pool: &Pool<Postgres>,
        user_id: i32,
        pages: &Pages,
        page: &Page,
    ) -> sqlx::Result<Vec<Self>> {
        sqlx::query_as::<_, Self>(
            "SELECT quotes.*,
               quotes.created_at AT TIME ZONE 'UTC' AS created_at,
               quotes.updated_at AT TIME ZONE 'UTC' AS updated_at,
               (SELECT COUNT(*) FROM comments WHERE comments.quote_id = quotes.id) AS comments_count,
               quoter.username AS quoter_username,
               quoter.fullname AS quoter_fullname,
               quoter.email_address AS quoter_email_address,
               quoter.openid AS quoter_openid,
               quotee.username AS quotee_username,
               quotee.fullname AS quotee_fullname,
               quotee.email_address AS quotee_email_address,
               quotee.openid AS quotee_openid,
               contexts.name AS context_name,
               contexts.description AS context_description
             FROM quotes
               INNER JOIN users AS quoter ON quoter.id = quoter_id
               INNER JOIN users AS quotee ON quotee.id = quotee_id
               INNER JOIN contexts ON contexts.id = context_id
               INNER JOIN contexts_users ON contexts_users.context_id = quotes.context_id
             WHERE NOT hidden AND contexts_users.user_id = $1
             ORDER BY quotes.created_at DESC
             LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(pages.limit() as i64)
        .bind(page.start as i64)
        .fetch_all(pool)
        .await
    }
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
