use crate::{
    errors::InternalError,
    filters,
    types::{Context, Flash, QuoteWithUsers, User},
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
        users,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "users/index.html")]
struct IndexTemplate {
    flash: Flash,
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
    let quotes = sqlx::query_as::<_, QuoteWithUsers>(
            "SELECT quotes.*,
               (SELECT COUNT(*) FROM comments WHERE comments.quote_id = quotes.id) AS comments_count,
               quoter.username AS quoter_username,
               quoter.fullname AS quoter_fullname,
               quoter.email_address AS quoter_email_address,
               quoter.openid AS quoter_openid,
               quotee.username AS quotee_username,
               quotee.fullname AS quotee_fullname,
               quotee.email_address AS quotee_email_address,
               quotee.openid AS quotee_openid
             FROM quotes
               INNER JOIN users AS quoter ON quoter.id = quoter_id
               INNER JOIN users AS quotee ON quotee.id = quotee_id
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
        user,
        quotes,
        comments: vec!["Comment".to_string()],
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
    user: User,
    quotes: Vec<QuoteWithUsers>,
    comments: Vec<String>,
    contexts: Vec<Context>,
}
