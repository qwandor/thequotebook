use pulldown_cmark::{
    escape::{escape_href, escape_html},
    Event, Parser, Tag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AllowedTags {
    pub emphasis: bool,
    pub strong: bool,
    pub link: bool,
}

impl AllowedTags {
    pub const ALL: Self = Self {
        emphasis: true,
        strong: true,
        link: true,
    };
}

pub fn markdown_to_html(
    markdown: &str,
    newlines_allowed: bool,
    allowed_tags: &AllowedTags,
) -> String {
    let parser = Parser::new(markdown);

    to_html(parser, newlines_allowed, allowed_tags)
}

fn to_html<'a>(
    events: impl Iterator<Item = Event<'a>>,
    newlines_allowed: bool,
    allowed_tags: &AllowedTags,
) -> String {
    let mut result = String::new();
    let mut between_paragraphs = false;
    for event in events {
        match event {
            Event::Start(tag) => {
                if allowed(&tag, allowed_tags) {
                    start(&mut result, &mut between_paragraphs, &tag, newlines_allowed)
                }
            }
            Event::End(tag) => {
                if allowed(&tag, allowed_tags) {
                    end(&mut result, &mut between_paragraphs, &tag)
                }
            }
            Event::Text(text) => escape_html(&mut result, &text).unwrap(),
            Event::Code(text) => {
                result += "`";
                escape_html(&mut result, &text).unwrap();
                result += "`";
            }
            _ => {}
        }
    }
    result
}

fn allowed(tag: &Tag, allowed_tags: &AllowedTags) -> bool {
    match tag {
        Tag::Emphasis => allowed_tags.emphasis,
        Tag::Strong => allowed_tags.strong,
        Tag::Link(_, _, _) => allowed_tags.link,
        Tag::Paragraph => true,
        _ => false,
    }
}

fn start(buffer: &mut String, between_paragraphs: &mut bool, tag: &Tag, newlines_allowed: bool) {
    match tag {
        Tag::Emphasis => *buffer += "<em>",
        Tag::Strong => *buffer += "<strong>",
        Tag::Paragraph => {
            if *between_paragraphs {
                *buffer += if newlines_allowed { "<br/><br/>" } else { " " };
            }
        }
        Tag::Link(link_type, url, title) => {
            *buffer += "<a href=\"";
            escape_href(&mut *buffer, url).unwrap();
            *buffer += "\"";
            if !title.is_empty() {
                *buffer += " title=\"";
                escape_html(&mut *buffer, title).unwrap();
                *buffer += "\"";
            }
            *buffer += ">";
        }
        _ => {}
    }
}

fn end(buffer: &mut String, between_paragraphs: &mut bool, tag: &Tag) {
    match tag {
        Tag::Emphasis => *buffer += "</em>",
        Tag::Strong => *buffer += "</strong>",
        Tag::Paragraph => *between_paragraphs = true,
        Tag::Link(_, _, _) => *buffer += "</a>",
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(
            markdown_to_html("", false, &AllowedTags::ALL),
            "".to_string()
        );
        assert_eq!(
            markdown_to_html("", true, &AllowedTags::ALL),
            "".to_string()
        );
    }

    #[test]
    fn basic_formatting() {
        assert_eq!(
            markdown_to_html("Text _italic_ and **bold**", false, &AllowedTags::ALL),
            "Text <em>italic</em> and <strong>bold</strong>".to_string()
        );
    }

    #[test]
    fn links() {
        assert_eq!(
            markdown_to_html("some [link](http://blah.blah)", false, &AllowedTags::ALL),
            "some <a href=\"http://blah.blah\">link</a>".to_string()
        );
        assert_eq!(
            markdown_to_html(
                "some [link](http://blah.blah \"with a title\")",
                false,
                &AllowedTags::ALL
            ),
            "some <a href=\"http://blah.blah\" title=\"with a title\">link</a>".to_string()
        );
    }

    #[test]
    fn paragraphs() {
        assert_eq!(
            markdown_to_html("none", true, &AllowedTags::ALL),
            "none".to_string()
        );
        assert_eq!(
            markdown_to_html("one\ntwo\n\nthree", true, &AllowedTags::ALL),
            "one two<br/><br/>three".to_string()
        );
        assert_eq!(
            markdown_to_html("one\ntwo\n\nthree", false, &AllowedTags::ALL),
            "one two three".to_string()
        );
    }

    #[test]
    fn unsupported_formatting() {
        assert_eq!(
            markdown_to_html("We don't support `code`", false, &AllowedTags::ALL),
            "We don't support `code`".to_string()
        );
    }
}
