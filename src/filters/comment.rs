use crate::markdown::{markdown_to_html, AllowedTags};

pub fn comment_format(body: &str, newlines_allowed: bool) -> askama::Result<String> {
    Ok(markdown_to_html(body, newlines_allowed, &AllowedTags::ALL))
}
