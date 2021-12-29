use crate::{
    atom::{comments::comments_to_atom, quotes::quotes_to_atom},
    config::Config,
    errors::InternalError,
    filters,
    model::{CommentWithQuote, CommentWithQuotee, Context, QuoteWithUsers, User},
    pagination::{PageOrGap, PaginationState, QueryPage},
    responses::Atom,
    session::Session,
};
use askama::Template;
use axum::{
    extract::{Extension, Path, Query},
    response::Html,
};
use paginate::Pages;
use sqlx::{Pool, Postgres};
use std::sync::Arc;

const QUOTES_PER_PAGE: usize = 10;
const PAGINATION_WINDOW: usize = 2;

pub async fn index(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
) -> Result<Html<String>, InternalError> {
    let users = User::fetch_all(&pool).await?;
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
    let user = User::fetch_one(&pool, user_id).await?;
    let comments = CommentWithQuote::fetch_5_for_user(&pool, user_id).await?;

    let quote_count = QuoteWithUsers::count_for_quotee(&pool, user_id).await?;
    let pages = Pages::new(quote_count, QUOTES_PER_PAGE);
    let current_page = pages.with_offset(query.page);
    let quotes =
        QuoteWithUsers::fetch_page_for_quotee(&pool, user_id, &pages, &current_page).await?;
    let contexts = Context::fetch_for_user(&pool, user_id).await?;

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
    let user = User::fetch_one(&pool, user_id).await?;
    let quotes = QuoteWithUsers::fetch_all_for_quotee(&pool, user_id).await?;

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

pub async fn quotes_atom(
    Extension(config): Extension<Arc<Config>>,
    Extension(pool): Extension<Pool<Postgres>>,
    Path(user_id): Path<i32>,
) -> Result<Atom, InternalError> {
    let user = User::fetch_one(&pool, user_id).await?;
    let quotes = QuoteWithUsers::fetch_all_for_quotee(&pool, user_id).await?;
    let title = format!("theQuotebook: Quotes by {}", user.fullname);
    let path = format!("/users/{}/quotes", user_id);

    Ok(Atom(quotes_to_atom(quotes, title, &path, &config)?))
}

pub async fn relevant_quotes(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path(user_id): Path<i32>,
) -> Result<Html<String>, InternalError> {
    let user = User::fetch_one(&pool, user_id).await?;
    let quotes = QuoteWithUsers::fetch_all_for_user_contexts(&pool, user_id).await?;

    let template = RelevantQuotesTemplate {
        session,
        user,
        quotes,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "users/relevant_quotes.html")]
struct RelevantQuotesTemplate {
    session: Session,
    user: User,
    quotes: Vec<QuoteWithUsers>,
}

pub async fn relevant_quotes_atom(
    Extension(config): Extension<Arc<Config>>,
    Extension(pool): Extension<Pool<Postgres>>,
    Path(user_id): Path<i32>,
) -> Result<Atom, InternalError> {
    let user = User::fetch_one(&pool, user_id).await?;
    let quotes = QuoteWithUsers::fetch_all_for_user_contexts(&pool, user_id).await?;
    let title = format!("theQuotebook: Quotes of interest to {}", user.fullname);
    let path = format!("/users/{}/relevant_quotes", user_id);

    Ok(Atom(quotes_to_atom(quotes, title, &path, &config)?))
}

pub async fn relevant_comments(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path(user_id): Path<i32>,
) -> Result<Html<String>, InternalError> {
    let user = User::fetch_one(&pool, user_id).await?;
    let comments = CommentWithQuotee::fetch_all_for_user_contexts(&pool, user_id).await?;

    let template = RelevantCommentsTemplate {
        session,
        user,
        comments,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "users/relevant_comments.html")]
struct RelevantCommentsTemplate {
    session: Session,
    user: User,
    comments: Vec<CommentWithQuotee>,
}

pub async fn relevant_comments_atom(
    Extension(config): Extension<Arc<Config>>,
    Extension(pool): Extension<Pool<Postgres>>,
    Path(user_id): Path<i32>,
) -> Result<Atom, InternalError> {
    let user = User::fetch_one(&pool, user_id).await?;
    let comments = CommentWithQuotee::fetch_all_for_user_contexts(&pool, user_id).await?;
    let title = format!("theQuotebook: Comments of interest to {}", user.fullname);
    let path = format!("/users/{}/relevant_comments", user_id);

    Ok(Atom(comments_to_atom(comments, title, &path, &config)?))
}

pub async fn edit(
    session: Session,
    Path(user_id): Path<i32>,
) -> Result<Html<String>, InternalError> {
    // There must be a user logged in, and they can only edit their own profile.
    let user = session
        .current_user
        .clone()
        .ok_or(InternalError::Unauthorised)?;
    if user.id != user_id {
        return Err(InternalError::Unauthorised);
    }

    let template = EditTemplate {
        session,
        user,
        form: UserForm {
            error_messages: "".to_string(),
        },
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "users/edit.html")]
struct EditTemplate {
    session: Session,
    user: User,
    form: UserForm,
}

struct UserForm {
    error_messages: String,
}
