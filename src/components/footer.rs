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
            p {
                "© 2023 Joey Lent. MIT License. | "
                a href="https://github.com/Supercolbat/website" { "v" (env!("CARGO_PKG_VERSION")) }
            }
        }
    }
}
