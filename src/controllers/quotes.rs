use crate::{
    config::Config,
    errors::InternalError,
    filters::{self, chatty_quote},
    model::{CommentWithQuote, Context, QuoteWithUsers},
    responses::Atom,
    session::Session,
};
use askama::Template;
use atom_syndication::{
    ContentBuilder, Entry, EntryBuilder, FeedBuilder, GeneratorBuilder, LinkBuilder, PersonBuilder,
};
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
    // TODO: Should we sort by updated_at rather than created_at?
    let quotes = QuoteWithUsers::fetch_all(&pool).await?;

    let feed_url = config.absolute_url("/quotes.atom");
    let feed = FeedBuilder::default()
        .title("theQuotebook: All quotes")
        .link(
            LinkBuilder::default()
                .rel("self")
                .mime_type("application/atom+xml".to_string())
                .href(&feed_url)
                .build(),
        )
        .link(
            LinkBuilder::default()
                .rel("alternate")
                .mime_type("text/html".to_string())
                .href(config.absolute_url("/quotes"))
                .build(),
        )
        .id(feed_url)
        .generator(
            GeneratorBuilder::default()
                .value("theQuotebook")
                .uri(config.absolute_url("/"))
                .build(),
        )
        .entries(
            quotes
                .into_iter()
                .map(|quote| quote_to_atom(&config.base_url, quote))
                .collect::<Result<Vec<_>, InternalError>>()?,
        )
        .build();
    // TODO: Set updated timestamp.

    Ok(Atom(feed))
}

fn quote_to_atom(base_url: &str, quote: QuoteWithUsers) -> Result<Entry, InternalError> {
    let url = format!("{}/quotes/{}", base_url, quote.quote.id);
    Ok(EntryBuilder::default()
        .title(format!(
            "{}: {}",
            quote.quotee.fullname, quote.quote.quote_text
        ))
        .link(
            LinkBuilder::default()
                .rel("alternate")
                .mime_type("text/html".to_string())
                .href(&url)
                .build(),
        )
        .id(url)
        .updated(quote.quote.updated_at)
        .published(Some(quote.quote.created_at.into()))
        .author(
            PersonBuilder::default()
                .name(quote.quoter.username_or_fullname())
                .uri(format!("{}/users/{}", base_url, quote.quoter.id))
                .build(),
        )
        .content(
            ContentBuilder::default()
                .content_type("html".to_string())
                .value(chatty_quote(quote, base_url)?)
                .build(),
        )
        .build())
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
struct ShowTemplate {
    session: Session,
    quote: QuoteWithUsers,
    comments: Vec<CommentWithQuote>,
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
