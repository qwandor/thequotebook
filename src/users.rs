use super::filters;
use super::types::{Flash, User};
use askama::Template;
use axum::{extract::Extension, response::Html};
use sqlx::{Pool, Postgres};

pub async fn index(Extension(pool): Extension<Pool<Postgres>>) -> Result<Html<String>, String> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;
    let template = IndexTemplate {
        flash: Flash {
            notice: None,
            error: None,
        },
        users,
    };
    Ok(Html(template.render().map_err(|e| e.to_string())?))
}

#[derive(Template)]
#[template(path = "users/index.html")]
struct IndexTemplate {
    flash: Flash,
    users: Vec<User>,
}
