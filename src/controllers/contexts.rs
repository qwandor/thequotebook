use crate::{
    errors::InternalError,
    filters,
    session::Session,
    types::{CommentWithQuote, Context, QuoteWithUsers, User},
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
    let contexts = sqlx::query_as::<_, Context>(
        "SELECT contexts.*,
           (SELECT COUNT(*) FROM quotes WHERE quotes.context_id = contexts.id) as quotes_count
         FROM contexts
         ORDER BY contexts.created_at DESC",
    )
    .fetch_all(&pool)
    .await?;

    let template = IndexTemplate { session, contexts };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "contexts/index.html")]
struct IndexTemplate {
    session: Session,
    contexts: Vec<Context>,
}

pub async fn show(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path(context_id): Path<i32>,
) -> Result<Html<String>, InternalError> {
    let context = sqlx::query_as::<_, Context>(
        "SELECT contexts.*,
           (SELECT COUNT(*) FROM quotes WHERE quotes.context_id = contexts.id) as quotes_count
        FROM contexts WHERE id = $1",
    )
    .bind(context_id)
    .fetch_one(&pool)
    .await?;
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
         WHERE quotes.context_id = $1 AND NOT hidden
         ORDER BY quotes.created_at DESC",
    )
    .bind(context_id)
    .fetch_all(&pool)
    .await?;
    let users = sqlx::query_as::<_, User>(
        "SELECT users.*
         FROM users
           INNER JOIN contexts_users ON user_id = users.id
           WHERE context_id = $1
           ORDER BY users.created_at DESC",
    )
    .bind(context_id)
    .fetch_all(&pool)
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
             WHERE quotes.context_id = $1
             ORDER BY comments.created_at DESC LIMIT 5",
    )
    .bind(context_id)
    .fetch_all(&pool)
    .await?;

    let template = ShowTemplate {
        session,
        context,
        quotes,
        users,
        comments,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "contexts/show.html")]
struct ShowTemplate {
    session: Session,
    context: Context,
    quotes: Vec<QuoteWithUsers>,
    users: Vec<User>,
    comments: Vec<CommentWithQuote>,
}
