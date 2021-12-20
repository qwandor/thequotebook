use crate::{
    errors::InternalError,
    filters,
    model::{CommentWithQuote, CommentWithQuotee, Context, QuoteWithUsers},
    session::Session,
};
use askama::Template;
use axum::{
    extract::{Extension, Query},
    response::Html,
};
use paginate::{Page, Pages};
use serde::Deserialize;
use sqlx::{Pool, Postgres};

const QUOTES_PER_PAGE: usize = 5;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct QueryPage {
    #[serde(default)]
    pub page: usize,
}

pub async fn index(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Query(query): Query<QueryPage>,
) -> Result<Html<String>, InternalError> {
    let quote_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM quotes WHERE NOT hidden")
        .fetch_one(&pool)
        .await? as usize;
    let pages = Pages::new(quote_count, QUOTES_PER_PAGE);
    let current_page = pages.with_offset(query.page);
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
         WHERE NOT hidden
         ORDER BY quotes.created_at DESC
         LIMIT $1 OFFSET $2",
    )
    .bind(pages.limit() as i64)
    .bind(current_page.start as i64)
    .fetch_all(&pool)
    .await?;
    let top_contexts = sqlx::query_as::<_, Context>(
        "SELECT contexts.*,
           (SELECT COUNT(*) FROM quotes WHERE quotes.context_id = contexts.id) AS quotes_count
         FROM contexts
         ORDER BY quotes_count DESC LIMIT 5",
    )
    .fetch_all(&pool)
    .await?;
    let template = IndexTemplate {
        session,
        quotes,
        top_contexts,
        current_user_contexts: vec![],
        comments: vec![],
        pagination: PaginationState {
            pages,
            current_page,
        },
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "home/index.html")]
struct IndexTemplate {
    session: Session,
    quotes: Vec<QuoteWithUsers>,
    top_contexts: Vec<Context>,
    current_user_contexts: Vec<Context>,
    comments: Vec<CommentWithQuote>,
    pagination: PaginationState,
}

struct PaginationState {
    pages: Pages,
    current_page: Page,
}

pub async fn comments(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
) -> Result<Html<String>, InternalError> {
    let comments = sqlx::query_as::<_, CommentWithQuotee>(
        "SELECT comments.*,
           comments.created_at AT TIME ZONE 'UTC' AS created_at,
           quotes.quote_text,
           quotes.quotee_id,
           users.email_address AS user_email_address,
           users.username AS user_username,
           users.fullname AS user_fullname,
           users.openid AS user_openid,
           quotee.username AS quotee_username,
           quotee.fullname AS quotee_fullname,
           quotee.email_address AS quotee_email_address,
           quotee.openid AS quotee_openid
         FROM comments
           INNER JOIN quotes ON quotes.id = comments.quote_id
           INNER JOIN users ON users.id = comments.user_id
           INNER JOIN users AS quotee ON quotee.id = quotes.quotee_id
         ORDER BY comments.created_at DESC",
    )
    .fetch_all(&pool)
    .await?;

    let template = CommentsTemplate { session, comments };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "home/comments.html")]
struct CommentsTemplate {
    session: Session,
    comments: Vec<CommentWithQuotee>,
}
