use super::quotes::{self, QuoteForm};
use crate::{
    atom::quotes::quotes_to_atom,
    config::Config,
    errors::InternalError,
    filters,
    model::{CommentWithQuote, Context, QuoteWithUsers, User},
    pagination::{PageOrGap, PaginationState, QueryPage},
    responses::Atom,
    session::Session,
};
use askama::Template;
use axum::{
    extract::{Extension, Path, Query},
    response::{Html, Redirect},
};
use paginate::Pages;
use sqlx::{Pool, Postgres};
use std::sync::Arc;
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
        form: QuoteForm {
            context: Some(context),
            ..QuoteForm::default()
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
    form: QuoteForm,
}

pub async fn new(session: Session) -> Result<Html<String>, InternalError> {
    // There must be a user logged in.
    if !session.logged_in() {
        return Err(InternalError::Unauthorised);
    }

    let template = NewTemplate {
        session,
        form: ContextForm::default(),
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "contexts/new.html")]
struct NewTemplate {
    session: Session,
    form: ContextForm,
}

#[derive(Clone, Debug, Default)]
struct ContextForm {
    error_messages: String,
    name: String,
    description: String,
}

impl From<Context> for ContextForm {
    fn from(context: Context) -> Self {
        Self {
            error_messages: "".to_string(),
            name: context.name,
            description: context.description,
        }
    }
}

pub async fn edit(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path(context_id): Path<i32>,
) -> Result<Html<String>, InternalError> {
    let context = Context::fetch_one(&pool, context_id).await?;

    // There must be a user logged in.
    if !session.logged_in() {
        return Err(InternalError::Unauthorised);
    }

    let template = EditTemplate {
        session,
        form: context.into(),
        context_id,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "contexts/edit.html")]
struct EditTemplate {
    session: Session,
    form: ContextForm,
    context_id: i32,
}

pub async fn latest(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path(context_id): Path<i32>,
) -> Result<Html<String>, InternalError> {
    let quote = QuoteWithUsers::fetch_latest_for_context(&pool, context_id).await?;
    let comments = CommentWithQuote::fetch_all_for_quote(&pool, quote.quote.id).await?;

    let template = quotes::ShowTemplate {
        session,
        quote,
        comments,
    };
    Ok(Html(template.render()?))
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

    Ok(Redirect::to(&format!("/contexts/{}", context_id)))
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

    Ok(Redirect::to(&format!("/contexts/{}", context_id)))
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

pub async fn quotes_atom(
    Extension(config): Extension<Arc<Config>>,
    Extension(pool): Extension<Pool<Postgres>>,
    Path(context_id): Path<i32>,
) -> Result<Atom, InternalError> {
    let context = Context::fetch_one(&pool, context_id).await?;
    let quotes = QuoteWithUsers::fetch_all_for_context(&pool, context_id).await?;
    let title = format!("theQuotebook: {} quotes", context.name);
    let path = format!("/contexts/{}/quotes", context_id);

    Ok(Atom(quotes_to_atom(quotes, title, &path, &config)?))
}
