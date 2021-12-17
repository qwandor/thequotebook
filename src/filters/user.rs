use super::escape;
use crate::types::User;

pub fn link_to_user(
    user: &User,
    &avatar: &bool,
    &actually_link: &bool,
    &css_avatar: &bool,
    &avatar_size: &u16,
    prefix: &str,
) -> askama::Result<String> {
    let content_text = user.username.as_ref().unwrap_or(&user.fullname);
    let title = &user.fullname;
    let mut class = "nickname";
    let mut style = "".to_string();
    let email_address = user.email_address.as_deref().unwrap_or("");
    let gravatar = if avatar && !css_avatar {
        gravatar(email_address, avatar_size)
    } else {
        "".to_string()
    };
    let link_text = format!("{}{}{}", prefix, gravatar, escape(content_text));

    if css_avatar {
        let image = gravatar_url(email_address, avatar_size);
        class = "author";
        style = format!("background-image:url({})", escape(&image));
    }

    Ok(if actually_link {
        // TODO: Support full_url
        let user_url = format!("/user/{}", user.id);
        format!(
            "<a href=\"{}\" class=\"{}\" title=\"{}\" style=\"{}\">{}</a>",
            user_url, class, title, style, link_text
        )
    } else if css_avatar {
        format!(
            "<span class=\"{}\" style=\"{}\">{}</span>",
            class, style, link_text
        )
    } else {
        link_text
    })
}

fn gravatar(email_address: &str, avatar_size: u16) -> String {
    let url = gravatar_url(email_address, avatar_size);
    format!(
        "<img class=\"gravatar\" alt=\"avatar\" width=\"{}\" height=\"{}\" src=\"{}\"/>",
        avatar_size,
        avatar_size,
        escape(&url)
    )
}

fn gravatar_url(email_address: &str, avatar_size: u16) -> String {
    let email_hash = md5::compute(email_address);
    format!(
        "http://www.gravatar.com/avatar.php?gravatar_id={:x}&size={}&rating=PG",
        email_hash, avatar_size
    )
}
