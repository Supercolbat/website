use maud::{html, Markup, PreEscaped};
use crate::icons::Icon;

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
