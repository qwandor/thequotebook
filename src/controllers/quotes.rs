use crate::{
    atom::quotes::quotes_to_atom,
    config::Config,
    errors::InternalError,
    filters::{self},
    model::{CommentWithQuote, Context, QuoteWithUsers},
    responses::Atom,
    session::Session,
};
use askama::Template;
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use sqlx::{Pool, Postgres};
use std::sync::Arc;

pub async fn index(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
) -> Result<Html<String>, InternalError> {
    let quotes = QuoteWithUsers::fetch_all(&pool).await?;

    let template = IndexTemplate { session, quotes };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "quotes/index.html")]
struct IndexTemplate {
    session: Session,
    quotes: Vec<QuoteWithUsers>,
}

pub async fn index_atom(
    Extension(config): Extension<Arc<Config>>,
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Atom, InternalError> {
    let quotes = QuoteWithUsers::fetch_all(&pool).await?;
    let title = "theQuotebook: All quotes".to_string();

    Ok(Atom(quotes_to_atom(quotes, title, "/quotes", &config)?))
}

pub async fn show(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path(quote_id): Path<i32>,
) -> Result<Html<String>, InternalError> {
    let quote = QuoteWithUsers::fetch_one(&pool, quote_id).await?;
    let comments = CommentWithQuote::fetch_all_for_quote(&pool, quote_id).await?;

    let template = ShowTemplate {
        session,
        quote,
        comments,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "quotes/show.html")]
pub struct ShowTemplate {
    pub session: Session,
    pub quote: QuoteWithUsers,
    pub comments: Vec<CommentWithQuote>,
}

pub async fn new(session: Session) -> Result<Html<String>, InternalError> {
    // There must be a user logged in.
    if !session.logged_in() {
        return Err(InternalError::Unauthorised);
    }

    let template = NewTemplate {
        session,
        form: QuoteForm::default(),
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "quotes/new.html")]
struct NewTemplate {
    session: Session,
    form: QuoteForm,
}

#[derive(Clone, Debug, Default)]
pub struct QuoteForm {
    pub error_messages: String,
    pub possible_quotee_matches: Option<Vec<String>>,
    pub quotee: String,
    pub context_name: String,
    pub context: Option<Context>,
    pub quote_text: String,
}

impl From<QuoteWithUsers> for QuoteForm {
    fn from(quote: QuoteWithUsers) -> Self {
        Self {
            error_messages: String::default(),
            possible_quotee_matches: None,
            quotee: quote.quotee.fullname,
            context_name: quote.context.name.clone(),
            context: Some(quote.context),
            quote_text: quote.quote.quote_text,
        }
    }
}

pub async fn edit(
    Extension(pool): Extension<Pool<Postgres>>,
    session: Session,
    Path(quote_id): Path<i32>,
) -> Result<Html<String>, InternalError> {
    let quote = QuoteWithUsers::fetch_one(&pool, quote_id).await?;

    // There must be a user logged in, and they can only edit their own quotes.
    let user = session
        .current_user
        .clone()
        .ok_or(InternalError::Unauthorised)?;
    if user.id != quote.quoter.id {
        return Err(InternalError::Unauthorised);
    }

    let template = EditTemplate {
        session,
        form: quote.into(),
        quote_id,
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "quotes/edit.html")]
struct EditTemplate {
    session: Session,
    form: QuoteForm,
    quote_id: i32,
}
