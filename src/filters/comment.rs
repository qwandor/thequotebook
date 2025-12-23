use crate::markdown::{markdown_to_html, AllowedTags};
use askama::{filter_fn, Values};

#[filter_fn]
pub fn comment_format(
    body: &str,
    _values: &dyn Values,
    newlines_allowed: bool,
) -> askama::Result<String> {
    Ok(markdown_to_html(body, newlines_allowed, &AllowedTags::ALL))
}
