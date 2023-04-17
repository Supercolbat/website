use maud::{html, Markup, PreEscaped};

/// Generate a card.
///
/// Style located in `sass/components/_footer.scss`.
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

            // Props to Project Segfault
            p {
                "Hosted on "
                a href="https://web.dev.projectsegfau.lt/pubnix" { "Project Segfault's Pubnix" }
            }

            // Copyright :)
            p { "© 2023 Joey Lent. MIT License." }
        }
    }
}
