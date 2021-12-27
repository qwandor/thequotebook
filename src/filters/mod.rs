mod comment;
mod quote;
mod time;
mod user;

use askama::Html;
pub use comment::comment_format;
pub use quote::{
    chatty_quote, comment_title_quote, formatted_quote, formatted_single_quote,
    quote_marks_if_needed, short_quote, tweet_quote_text,
};
use regex::Regex;
pub use time::long_datetime;
pub use user::gravatar_for;
pub use user::link_to_user;

fn escape(text: &str) -> String {
    askama::filters::escape(Html, text).unwrap().to_string()
}

fn bbcode_to_html(bbcode: &str, newlines_allowed: bool) -> String {
    //TODO
    if newlines_allowed {
        bbcode.to_string()
    } else {
        let newlines = Regex::new(r"[\r\n]+").unwrap();
        newlines.replace_all(bbcode, " ").into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape() {
        assert_eq!(escape("<a>&b"), "&lt;a&gt;&amp;b");
        assert_eq!(escape("\"'"), "&quot;&#x27;");
    }
}
