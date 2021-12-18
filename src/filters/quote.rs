use crate::types::{Context, Quote, QuoteWithUsers, User};
use askama::Template;

fn quote_marks_if_needed(text: &str) -> String {
    if text.contains('"') {
        text.to_string()
    } else {
        format!("\"{}\"", text)
    }
}

fn bbcode_to_html(bbcode: String) -> String {
    //TODO
    bbcode
}

pub fn formatted_quote(
    &quote: &&QuoteWithUsers,
    &single: &bool,
    &quoter_link: &bool,
    &quotee_link: &bool,
    &show_context: &bool,
    &show_comments: &bool,
) -> askama::Result<String> {
    let quote_link = !single;
    let text = bbcode_to_html(quote_marks_if_needed(&quote.quote.quote_text));
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

// Some filters need to be in scope for the Template derive macro above.
mod filters {
    pub use super::super::link_to_user;
    pub use super::super::long_datetime;
}
