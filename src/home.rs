use super::filters;
use super::types::{Context, Flash, Quote};
use askama::Template;
use axum::{extract::Extension, response::Html};
use sqlx::{Pool, Postgres};

pub async fn index(Extension(pool): Extension<Pool<Postgres>>) -> Result<Html<String>, String> {
    let quotes = sqlx::query_as::<_, Quote>(
        "SELECT * FROM quotes WHERE NOT hidden ORDER BY created_at DESC LIMIT 5",
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
    quotes: Vec<Quote>,
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
