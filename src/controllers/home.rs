use crate::{
    errors::InternalError,
    filters,
    session::Session,
    types::{Context, QuoteWithUsers},
};
use askama::Template;
use axum::{extract::Extension, response::Html};
use sqlx::{Pool, Postgres};

pub async fn index(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
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
         WHERE NOT hidden ORDER BY quotes.created_at DESC LIMIT 5",
    )
    .fetch_all(&pool)
    .await?;
    let top_contexts = sqlx::query_as::<_, Context>(
        "SELECT contexts.*,
           (SELECT COUNT(*) FROM quotes WHERE quotes.context_id = contexts.id) AS quotes_count
         FROM contexts
         ORDER BY quotes_count DESC LIMIT 5",
    )
    .fetch_all(&pool)
    .await?;
    let template = IndexTemplate {
        session,
        quotes,
        top_contexts,
        current_user_contexts: vec![],
        current_user_comments: vec![],
    };
    Ok(Html(template.render()?))
}

pub async fn comments(session: Session) -> Result<Html<String>, InternalError> {
    let template = CommentsTemplate { session };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "home/index.html")]
struct IndexTemplate {
    session: Session,
    quotes: Vec<QuoteWithUsers>,
    top_contexts: Vec<Context>,
    current_user_contexts: Vec<Context>,
    current_user_comments: Vec<String>,
}

#[derive(Template)]
#[template(path = "home/comments.html")]
struct CommentsTemplate {
    session: Session,
}
