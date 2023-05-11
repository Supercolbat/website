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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_footer() {
        let footer = footer();

        assert_eq!(
            footer.into_string(),
            r#"<footer><nav><a href="/blog">Blog</a>&middot;<a href="/rss">RSS</a>&middot;<a href="https://github.com/Supercolbat/website" target="_blank">Source</a></nav><p>Hosted on <a href="https://web.dev.projectsegfau.lt/pubnix">Project Segfault's Pubnix</a></p><p>© 2023 Joey Lent. MIT License.</p></footer>"#
        );
    }
}
