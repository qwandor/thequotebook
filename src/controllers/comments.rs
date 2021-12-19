use crate::{
    errors::InternalError,
    filters,
    types::{CommentWithQuote, Flash},
};
use askama::Template;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use sqlx::{Pool, Postgres};

pub async fn show(
    Extension(pool): Extension<Pool<Postgres>>,
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

    let template = ShowTemplate {
        flash: Flash {
            notice: None,
            error: None,
        },
        logged_in: false,
        comment,
        current_user_id: 1,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "comments/show.html")]
struct ShowTemplate {
    flash: Flash,
    logged_in: bool,
    comment: CommentWithQuote,
    current_user_id: i32,
}
