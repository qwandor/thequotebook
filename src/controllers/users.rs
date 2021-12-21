use crate::{
    errors::InternalError,
    filters,
    model::{CommentWithQuote, Context, QuoteWithUsers, User},
    pagination::{PageOrGap, PaginationState, QueryPage},
    session::Session,
};
use askama::Template;
use axum::{
    extract::{Extension, Path, Query},
    response::Html,
};
use paginate::Pages;
use sqlx::{Pool, Postgres};

const QUOTES_PER_PAGE: usize = 10;
const PAGINATION_WINDOW: usize = 2;

pub async fn index(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
) -> Result<Html<String>, InternalError> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&pool)
        .await?;
    let template = IndexTemplate { session, users };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "users/index.html")]
struct IndexTemplate {
    session: Session,
    users: Vec<User>,
}

pub async fn show(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path(user_id): Path<i32>,
    Query(query): Query<QueryPage>,
) -> Result<Html<String>, InternalError> {
    let user = User::fetch_one(&pool, user_id)
        .await?
        .ok_or(InternalError::NotFound)?;
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

    let quote_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM quotes WHERE quotes.quotee_id = $1 AND NOT hidden",
    )
    .bind(user_id)
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
             WHERE quotes.quotee_id = $1 AND NOT hidden
             ORDER BY quotes.created_at DESC
             LIMIT $2 OFFSET $3",
        )
        .bind(user_id)
        .bind(pages.limit() as i64)
        .bind(current_page.start as i64)
        .fetch_all(&pool)
        .await?;
    let contexts = sqlx::query_as::<_, Context>(
        "SELECT contexts.*,
          (SELECT COUNT(*) FROM quotes WHERE quotes.context_id = contexts.id) as quotes_count
         FROM contexts
           INNER JOIN contexts_users ON context_id = contexts.id
         WHERE user_id = $1
         ORDER BY contexts.created_at DESC",
    )
    .bind(user_id)
    .fetch_all(&pool)
    .await?;

    let template = ShowTemplate {
        session,
        user,
        quotes,
        comments,
        contexts,
        pagination: PaginationState {
            pages,
            current_page,
            window_size: PAGINATION_WINDOW,
        },
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "users/show.html")]
struct ShowTemplate {
    session: Session,
    user: User,
    quotes: Vec<QuoteWithUsers>,
    comments: Vec<CommentWithQuote>,
    contexts: Vec<Context>,
    pagination: PaginationState,
}

pub async fn quotes(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path(user_id): Path<i32>,
) -> Result<Html<String>, InternalError> {
    let user = User::fetch_one(&pool, user_id)
        .await?
        .ok_or(InternalError::NotFound)?;
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

    let template = QuotesTemplate {
        session,
        user,
        quotes,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "users/quotes.html")]
struct QuotesTemplate {
    session: Session,
    user: User,
    quotes: Vec<QuoteWithUsers>,
}
