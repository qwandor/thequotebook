use crate::{
    errors::InternalError,
    filters,
    types::{CommentWithQuote, Context, Flash, QuoteWithUsers, User},
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
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await?;
    let template = IndexTemplate {
        flash: Flash {
            notice: None,
            error: None,
        },
        logged_in: false,
        users,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "users/index.html")]
struct IndexTemplate {
    flash: Flash,
    logged_in: bool,
    users: Vec<User>,
}

pub async fn show(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(user_id): Path<i32>,
) -> Result<Html<String>, InternalError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
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
         WHERE comments.user_id = $1
         ORDER BY comments.created_at DESC LIMIT 5",
    )
    .bind(user_id)
    .fetch_all(&pool)
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
             WHERE quotes.quotee_id = $1 AND NOT hidden
             ORDER BY quotes.created_at DESC",
        )
        .bind(user_id)
        .fetch_all(&pool)
        .await?;

    let template = ShowTemplate {
        flash: Flash {
            notice: None,
            error: None,
        },
        logged_in: false,
        user,
        quotes,
        comments,
        contexts: vec![Context {
            id: 0,
            name: "Context".to_string(),
            description: "Description".to_string(),
            quote_count: 32,
        }],
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "users/show.html")]
struct ShowTemplate {
    flash: Flash,
    logged_in: bool,
    user: User,
    quotes: Vec<QuoteWithUsers>,
    comments: Vec<CommentWithQuote>,
    contexts: Vec<Context>,
}
