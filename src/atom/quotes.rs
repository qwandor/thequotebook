use crate::{config::Config, errors::InternalError, filters::chatty_quote, model::QuoteWithUsers};
use atom_syndication::{
    ContentBuilder, Entry, EntryBuilder, Feed, FeedBuilder, GeneratorBuilder, LinkBuilder,
    PersonBuilder,
};
use chrono::MIN_DATETIME;

pub fn quotes_to_atom(
    quotes: Vec<QuoteWithUsers>,
    title: String,
    config: &Config,
) -> Result<Feed, InternalError> {
    let last_updated = quotes
        .iter()
        .map(|quote| quote.quote.updated_at)
        .max()
        .unwrap_or(MIN_DATETIME);
    let feed_url = config.absolute_url("/quotes.atom");
    Ok(FeedBuilder::default()
        .title(title)
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
        .updated(last_updated)
        .entries(
            quotes
                .into_iter()
                .map(|quote| quote_to_atom(&config.base_url, quote))
                .collect::<Result<Vec<_>, InternalError>>()?,
        )
        .build())
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
