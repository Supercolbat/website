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
