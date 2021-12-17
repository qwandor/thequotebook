mod quote;
mod user;

use askama::Html;
pub use quote::formatted_quote;
pub use user::link_to_user;

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
