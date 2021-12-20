use crate::{
    errors::InternalError,
    filters,
    model::{CommentWithQuote, QuoteWithUsers},
    session::Session,
};
use askama::Template;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
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
         WHERE NOT hidden
         ORDER BY contexts.created_at DESC",
    )
    .fetch_all(&pool)
    .await?;

    let template = IndexTemplate { session, quotes };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "quotes/index.html")]
struct IndexTemplate {
    session: Session,
    quotes: Vec<QuoteWithUsers>,
}

pub async fn show(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path(quote_id): Path<i32>,
) -> Result<Html<String>, InternalError> {
    let quote = QuoteWithUsers::fetch_one(&pool, quote_id).await?;
    let comments = sqlx::query_as::<_, CommentWithQuote>(
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
    .fetch_all(&pool)
    .await?;

    let template = ShowTemplate {
        session,
        quote,
        comments,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "quotes/show.html")]
struct ShowTemplate {
    session: Session,
    quote: QuoteWithUsers,
    comments: Vec<CommentWithQuote>,
}
