use crate::{
    filters,
    types::{Flash, User, Quote},
};
use askama::Template;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
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

pub async fn show(
    Extension(pool): Extension<Pool<Postgres>>,
    Path(user_id): Path<i32>,
) -> Result<Html<String>, String> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())?;
    let template = ShowTemplate {
        flash: Flash {
            notice: None,
            error: None,
        },
        user,
        quotes: vec![],
    };
    Ok(Html(template.render().map_err(|e| e.to_string())?))
}

#[derive(Template)]
#[template(path = "users/show.html")]
struct ShowTemplate {
    flash: Flash,
    user: User,
    quotes: Vec<Quote>,
}
