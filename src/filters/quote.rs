use crate::{
    markdown::{markdown_to_html, AllowedTags},
    model::{Context, Quote, QuoteWithUsers, User},
};
use askama::{Template, Values, NO_VALUES};

const ALLOWED_TAGS: AllowedTags = AllowedTags {
    emphasis: true,
    strong: true,
    link: false,
};

pub fn quote_marks_if_needed(text: &str, _values: &dyn Values) -> askama::Result<String> {
    Ok(if text.contains('"') {
        text.to_string()
    } else {
        format!("\"{}\"", text)
    })
}

fn trim_if_needed(text: &str, max_length: usize) -> String {
    if text.chars().count() > max_length {
        format!(
            "{}...",
            text.chars().take(max_length - 3).collect::<String>()
        )
    } else {
        text.to_string()
    }
}

pub fn short_quote(text: &str, values: &dyn Values) -> askama::Result<String> {
    let trimmed = trim_if_needed(&quote_marks_if_needed(text, values)?, 40);
    Ok(markdown_to_html(&trimmed, false, &ALLOWED_TAGS))
}

pub fn comment_title_quote(text: &str, values: &dyn Values) -> askama::Result<String> {
    Ok(markdown_to_html(
        &quote_marks_if_needed(text, values)?,
        false,
        &ALLOWED_TAGS,
    ))
}

pub fn tweet_quote_text(text: &str, values: &dyn Values) -> askama::Result<String> {
    Ok(trim_if_needed(&quote_marks_if_needed(text, values)?, 80))
}

pub fn formatted_single_quote(
    quote: &QuoteWithUsers,
    values: &dyn Values,
) -> askama::Result<String> {
    formatted_quote(&quote, values, true, true, true, true, false)
}

pub fn formatted_quote(
    &quote: &&QuoteWithUsers,
    values: &dyn Values,
    single: bool,
    quoter_link: bool,
    quotee_link: bool,
    show_context: bool,
    show_comments: bool,
) -> askama::Result<String> {
    let quote_link = !single;
    let text = markdown_to_html(
        &quote_marks_if_needed(&quote.quote.quote_text, values)?,
        true,
        &ALLOWED_TAGS,
    );
    let comments_text = if show_comments {
        if quote.comments_count == 0 {
            "No comments (yet).".to_string()
        } else if quote.comments_count == 1 {
            "1 comment.".to_string()
        } else {
            format!("{} comments.", quote.comments_count)
        }
    } else {
        "".to_string()
    };

    let template = QuoteTemplate {
        quote: quote.quote.to_owned(),
        quoter: quote.quoter.to_owned(),
        quotee: quote.quotee.to_owned(),
        context: quote.context.to_owned(),
        single,
        quote_link,
        quoter_link,
        quotee_link,
        show_context,
        show_comments,
        text,
        comments_text,
    };
    template.render()
}

#[derive(Template)]
#[template(path = "shared/quote.html")]
struct QuoteTemplate {
    quote: Quote,
    quoter: User,
    quotee: User,
    context: Context,
    single: bool,
    quote_link: bool,
    quoter_link: bool,
    quotee_link: bool,
    show_context: bool,
    show_comments: bool,
    text: String,
    comments_text: String,
}

pub fn chatty_quote(quote: QuoteWithUsers, base_url: &str) -> askama::Result<String> {
    let text = markdown_to_html(
        &quote_marks_if_needed(&quote.quote.quote_text, NO_VALUES)?,
        true,
        &ALLOWED_TAGS,
    );

    let template = ChattyQuoteTemplate {
        quote: quote.quote.to_owned(),
        quoter: quote.quoter.to_owned(),
        quotee: quote.quotee.to_owned(),
        context: quote.context.to_owned(),
        text,
        base_url: base_url.to_owned(),
    };
    template.render()
}

#[derive(Template)]
#[template(path = "shared/chatty_quote.html")]
struct ChattyQuoteTemplate {
    quote: Quote,
    quoter: User,
    quotee: User,
    context: Context,
    text: String,
    base_url: String,
}

// Some filters need to be in scope for the Template derive macro above.
mod filters {
    pub use super::super::link_to_user;
    pub use super::super::long_datetime;
}
