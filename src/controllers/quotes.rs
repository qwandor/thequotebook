use crate::{
    errors::InternalError,
    filters,
    types::{Flash, QuoteWithUsers},
};
use askama::Template;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use sqlx::{Pool, Postgres};

pub async fn index(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Html<String>, InternalError> {
    let quotes = sqlx::query_as::<_, QuoteWithUsers>(
        "SELECT quotes.*,
		   quotes.created_at AT TIME ZONE 'UTC' AS created_at,
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
    .fetch_all(&pool)
    .await?;

    let template = IndexTemplate {
        flash: Flash {
            notice: None,
            error: None,
        },
        logged_in: false,
        quotes,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "quotes/index.html")]
struct IndexTemplate {
    flash: Flash,
    logged_in: bool,
    quotes: Vec<QuoteWithUsers>,
}
