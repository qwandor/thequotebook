use super::bbcode_to_html;

pub fn comment_format(body: &str) -> askama::Result<String> {
    Ok(bbcode_to_html(body, false))
}
