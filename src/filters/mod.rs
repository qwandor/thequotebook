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
pub use time::long_datetime;
pub use user::{gravatar_for, link_to_user};

fn escape(text: &str) -> String {
    askama::filters::escape(Html, text).unwrap().to_string()
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
