use super::quotes::AddQuoteForm;
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
    response::{Html, Redirect},
};
use paginate::Pages;
use sqlx::{Pool, Postgres};
use tower_cookies::{Cookie, Cookies};

const QUOTES_PER_PAGE: usize = 10;
const PAGINATION_WINDOW: usize = 2;

pub async fn index(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
) -> Result<Html<String>, InternalError> {
    let contexts = Context::fetch_all(&pool).await?;

    let template = IndexTemplate { session, contexts };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "contexts/index.html")]
struct IndexTemplate {
    session: Session,
    contexts: Vec<Context>,
}

pub async fn show(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path(context_id): Path<i32>,
    Query(query): Query<QueryPage>,
) -> Result<Html<String>, InternalError> {
    let context = Context::fetch_one(&pool, context_id).await?;

    let quote_count = QuoteWithUsers::count_for_context(&pool, context_id).await?;
    let pages = Pages::new(quote_count, QUOTES_PER_PAGE);
    let current_page = pages.with_offset(query.page);
    let quotes =
        QuoteWithUsers::fetch_page_for_context(&pool, context_id, &pages, &current_page).await?;
    let users = User::fetch_all_for_context(&pool, context_id).await?;
    let comments = CommentWithQuote::fetch_5_for_context(&pool, context_id).await?;

    let template = ShowTemplate {
        session,
        context: context.clone(),
        quotes,
        users,
        comments,
        pagination: PaginationState {
            pages,
            current_page,
            window_size: PAGINATION_WINDOW,
        },
        form: AddQuoteForm {
            error_messages: "".to_string(),
            possible_quotee_matches: None,
            quotee: "".to_string(),
            context_name: "".to_string(),
            context: Some(context),
        },
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "contexts/show.html")]
struct ShowTemplate {
    session: Session,
    context: Context,
    quotes: Vec<QuoteWithUsers>,
    users: Vec<User>,
    comments: Vec<CommentWithQuote>,
    pagination: PaginationState,
    form: AddQuoteForm,
}

pub async fn new(session: Session) -> Result<Html<String>, InternalError> {
    let template = NewTemplate {
        session,
        form: NewContextForm {
            error_messages: "".to_string(),
        },
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "contexts/new.html")]
struct NewTemplate {
    session: Session,
    form: NewContextForm,
}

struct NewContextForm {
    error_messages: String,
}

pub async fn join(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path(context_id): Path<i32>,
    cookies: Cookies,
) -> Result<Redirect, InternalError> {
    let current_user = session.current_user.ok_or(InternalError::Unauthorised)?;
    let context = Context::fetch_one(&pool, context_id).await?;

    User::join_context(&pool, current_user.id, context_id).await?;
    cookies.add(Cookie::new(
        "notice",
        format!("You are now a member of {}.", context.name),
    ));

    Ok(Redirect::to(
        format!("/contexts/{}", context_id).parse().unwrap(),
    ))
}

pub async fn leave(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path(context_id): Path<i32>,
    cookies: Cookies,
) -> Result<Redirect, InternalError> {
    let current_user = session.current_user.ok_or(InternalError::Unauthorised)?;
    let context = Context::fetch_one(&pool, context_id).await?;

    User::leave_context(&pool, current_user.id, context_id).await?;
    cookies.add(Cookie::new(
        "notice",
        format!("You are no longer a member of {}.", context.name),
    ));

    Ok(Redirect::to(
        format!("/contexts/{}", context_id).parse().unwrap(),
    ))
}

pub async fn quotes(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path(context_id): Path<i32>,
) -> Result<Html<String>, InternalError> {
    let context = Context::fetch_one(&pool, context_id).await?;
    let quotes = QuoteWithUsers::fetch_all_for_context(&pool, context_id).await?;

    let template = QuotesTemplate {
        session,
        context,
        quotes,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "contexts/quotes.html")]
struct QuotesTemplate {
    session: Session,
    context: Context,
    quotes: Vec<QuoteWithUsers>,
}
