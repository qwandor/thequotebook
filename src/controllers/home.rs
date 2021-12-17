use crate::{
    filters,
    types::{Context, Flash, QuoteWithUsers},
};
use askama::Template;
use axum::{extract::Extension, response::Html};
use sqlx::{Pool, Postgres};

pub async fn index(Extension(pool): Extension<Pool<Postgres>>) -> Result<Html<String>, String> {
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
         WHERE NOT hidden ORDER BY quotes.created_at DESC LIMIT 5",
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| e.to_string())?;
    let template = IndexTemplate {
        flash: Flash {
            notice: None,
            error: None,
        },
        logged_in: false,
        quotes,
        top_contexts: vec![],
        current_user_contexts: vec![],
        current_user_comments: vec![],
        current_user_id: 0,
        current_user_fullname: "".to_string(),
    };
    Ok(Html(template.render().map_err(|e| e.to_string())?))
}

pub async fn comments() -> Result<Html<String>, String> {
    let template = CommentsTemplate {
        flash: Flash {
            notice: None,
            error: None,
        },
    };
    Ok(Html(template.render().map_err(|e| e.to_string())?))
}

#[derive(Template)]
#[template(path = "home/index.html")]
struct IndexTemplate {
    flash: Flash,
    logged_in: bool,
    quotes: Vec<QuoteWithUsers>,
    top_contexts: Vec<Context>,
    current_user_contexts: Vec<Context>,
    current_user_comments: Vec<String>,
    current_user_id: u32,
    current_user_fullname: String,
}

#[derive(Template)]
#[template(path = "home/comments.html")]
struct CommentsTemplate {
    flash: Flash,
}
