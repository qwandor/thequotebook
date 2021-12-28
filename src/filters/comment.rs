use super::bbcode_to_html;

pub fn comment_format(body: &str, newlines_allowed: bool) -> askama::Result<String> {
    Ok(bbcode_to_html(body, newlines_allowed))
}
