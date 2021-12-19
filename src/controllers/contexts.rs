use crate::{
    errors::InternalError,
    types::{Context, Flash},
};
use askama::Template;
use axum::{extract::Extension, response::Html};
use sqlx::{Pool, Postgres};

pub async fn index(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Html<String>, InternalError> {
    let contexts = sqlx::query_as::<_, Context>(
        "SELECT contexts.*,
           (SELECT COUNT(*) FROM quotes WHERE quotes.context_id = contexts.id) as quotes_count
         FROM contexts
         ORDER BY contexts.created_at DESC",
    )
    .fetch_all(&pool)
    .await?;

    let template = IndexTemplate {
        flash: Flash {
            notice: None,
            error: None,
        },
        logged_in: false,
        contexts,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "contexts/index.html")]
struct IndexTemplate {
    flash: Flash,
    logged_in: bool,
    contexts: Vec<Context>,
}
