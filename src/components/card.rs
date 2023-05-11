use maud::{html, Markup, PreEscaped};
use crate::components::Icon;

/// Generate a card.
///
/// Style located in `sass/components/_card.scss`.
pub fn card(icon: Icon, title: &str, description: &str) -> Markup {
    let svg = icon.as_svg();
    html! {
        div.card {
            div {
                (PreEscaped(svg))
                span { (title) }
            }
            p { (description) }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::Icon;

    #[test]
    fn test_card() {
        let title = "Github";
        let description = "This is a card";
        let card = card(Icon::Github, title, description);

        assert_eq!(
            card.into_string(),
            format!(
                "<div class=\"card\"><div>{}<span>{}</span></div><p>{}</p></div>",
                Icon::Github.as_svg(),
                title,
                description,
            )
        );
    }
}
