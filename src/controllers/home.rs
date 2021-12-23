use crate::{
    errors::InternalError,
    filters,
    model::{CommentWithQuote, CommentWithQuotee, Context, QuoteWithUsers},
    pagination::{PageOrGap, PaginationState, QueryPage},
    session::Session,
};
use askama::Template;
use axum::{
    extract::{Extension, Query},
    response::Html,
};
use paginate::Pages;
use sqlx::{Pool, Postgres};

const QUOTES_PER_PAGE: usize = 5;
const PAGINATION_WINDOW: usize = 2;

pub async fn index(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Query(query): Query<QueryPage>,
) -> Result<Html<String>, InternalError> {
    let top_contexts = Context::fetch_top_5(&pool).await?;

    let template = if let Some(current_user) = &session.current_user {
        let quote_count = QuoteWithUsers::count_for_user_contexts(&pool, current_user.id).await?;
        let pages = Pages::new(quote_count, QUOTES_PER_PAGE);
        let current_page = pages.with_offset(query.page);
        let quotes = QuoteWithUsers::fetch_page_for_user_contexts(
            &pool,
            current_user.id,
            &pages,
            &current_page,
        )
        .await?;
        let current_user_contexts = Context::fetch_for_user(&pool, current_user.id).await?;
        let comments = CommentWithQuote::fetch_5_for_user_contexts(&pool, current_user.id).await?;

        IndexTemplate {
            session,
            quotes,
            top_contexts,
            current_user_contexts,
            comments,
            pagination: PaginationState {
                pages,
                current_page,
                window_size: PAGINATION_WINDOW,
            },
        }
    } else {
        let quote_count = QuoteWithUsers::count(&pool).await?;
        let pages = Pages::new(quote_count, QUOTES_PER_PAGE);
        let current_page = pages.with_offset(query.page);
        let quotes = QuoteWithUsers::fetch_page(&pool, &pages, &current_page).await?;

        IndexTemplate {
            session,
            quotes,
            top_contexts,
            current_user_contexts: vec![],
            comments: vec![],
            pagination: PaginationState {
                pages,
                current_page,
                window_size: PAGINATION_WINDOW,
            },
        }
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

pub async fn comments(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
) -> Result<Html<String>, InternalError> {
    let comments = CommentWithQuotee::fetch_all(&pool).await?;

    let template = CommentsTemplate { session, comments };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "home/comments.html")]
struct CommentsTemplate {
    session: Session,
    comments: Vec<CommentWithQuotee>,
}
