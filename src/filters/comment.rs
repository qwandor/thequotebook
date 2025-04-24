use crate::markdown::{markdown_to_html, AllowedTags};
use askama::Values;

pub fn comment_format(
    body: &str,
    _values: &dyn Values,
    newlines_allowed: bool,
) -> askama::Result<String> {
    Ok(markdown_to_html(body, newlines_allowed, &AllowedTags::ALL))
}
