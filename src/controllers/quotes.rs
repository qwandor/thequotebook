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
    let template = NewTemplate {
        session,
        form: AddQuoteForm {
            error_messages: "".to_string(),
            possible_quotee_matches: None,
            quotee: "".to_string(),
            context_name: "".to_string(),
            context: None,
        },
    };
    Ok(Html(template.render()?))
}

#[derive(Template)]
#[template(path = "quotes/new.html")]
struct NewTemplate {
    session: Session,
    form: AddQuoteForm,
}

pub struct AddQuoteForm {
    pub error_messages: String,
    pub possible_quotee_matches: Option<Vec<String>>,
    pub quotee: String,
    pub context_name: String,
    pub context: Option<Context>,
}
