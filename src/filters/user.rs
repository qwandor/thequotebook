use super::escape;
use crate::model::User;

pub fn link_to_user(
    user: &User,
    &avatar: &bool,
    &actually_link: &bool,
    &swap_names: &bool,
    &css_avatar: &bool,
    &avatar_size: &u16,
    prefix: &str,
    class: &str,
) -> askama::Result<String> {
    let username = user.username_or_fullname();
    let fullname: &str = &user.fullname;
    let (content_text, title) = if swap_names {
        (fullname, username)
    } else {
        (username, fullname)
    };
    let mut style = "".to_string();
    let email_address = user.email_address.as_deref().unwrap_or("");
    let gravatar = if avatar && !css_avatar {
        gravatar(email_address, avatar_size, "gravatar")
    } else {
        "".to_string()
    };
    let link_text = format!("{}{}{}", prefix, gravatar, escape(content_text));

    if css_avatar {
        let image = gravatar_url(email_address, avatar_size);
        style = format!("background-image:url({})", escape(&image));
    }

    Ok(if actually_link {
        // TODO: Support full_url
        let user_url = format!("/users/{}", user.id);
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

pub fn gravatar_for(user: &User, &avatar_size: &u16, class: &str) -> askama::Result<String> {
    let email_address = user.email_address.as_deref().unwrap_or("");
    Ok(gravatar(email_address, avatar_size, class))
}

fn gravatar(email_address: &str, avatar_size: u16, class: &str) -> String {
    let url = gravatar_url(email_address, avatar_size);
    format!(
        "<img class=\"{}\" alt=\"avatar\" width=\"{}\" height=\"{}\" src=\"{}\"/>",
        class,
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
