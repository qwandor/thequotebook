use crate::{
    errors::InternalError,
    filters,
    session::Session,
    types::{CommentWithQuote, Quote},
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
    Path(quote_id): Path<i32>,
) -> Result<Html<String>, InternalError> {
    let quote = sqlx::query_as::<_, Quote>(
        "SELECT quotes.*,
           quotes.created_at AT TIME ZONE 'UTC' AS created_at
         FROM quotes
         WHERE id = $1",
    )
    .bind(quote_id)
    .fetch_one(&pool)
    .await?;
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

    let template = IndexTemplate {
        session,
        quote,
        comments,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "comments/index.html")]
struct IndexTemplate {
    session: Session,
    quote: Quote,
    comments: Vec<CommentWithQuote>,
}

pub async fn show(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path((quote_id, comment_id)): Path<(i32, i32)>,
) -> Result<Html<String>, InternalError> {
    let comment = sqlx::query_as::<_, CommentWithQuote>(
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
    .fetch_one(&pool)
    .await?;

    let template = ShowTemplate { session, comment };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "comments/show.html")]
struct ShowTemplate {
    session: Session,
    comment: CommentWithQuote,
}
