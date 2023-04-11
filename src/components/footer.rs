use maud::{html, Markup, PreEscaped};

pub fn footer() -> Markup {
    html! {
        footer {
            // Links
            nav {
                a href="/blog" { "Blog" }
                (PreEscaped("&middot;"))
                a href="/contact" { "Contact" }
                (PreEscaped("&middot;"))
                a href="/rss" { "RSS" }
                (PreEscaped("&middot;"))
                a href="https://github.com/Supercolbat/website" target="_blank" { "Source" }
            }

            // Copyright :)
            p { "© 2023 Joey Lent. MIT License." }
        }
    }
}
