use maud::{html, Markup, PreEscaped};

pub fn footer() -> Markup {
    html! {
        footer {
            // Links
            p {
                a href="#" { "Blog" }
                (PreEscaped("&bull;"))
                a href="#" { "Contact" }
                (PreEscaped("&bull;"))
                a href="#" { "RSS" }
                (PreEscaped("&bull;"))
                a href="#" { "Source" }
            }

            // Copyright :)
            p { "Joey Lent | v0.1.0" }
        }
    }
}
