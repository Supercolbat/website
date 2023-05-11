use maud::{html, Markup};

/// Generate meta tags for mobile support and for embeds.
pub fn meta_tags(description: &str) -> Markup {
    html! {
        // Mobile support
        meta name="viewport"    content="width=device-width,initial-scale=1.0" {}

        // Primary Meta Tags
        meta name="title"       content="Joey Lent" {}
        meta name="description" content=(description) {}

        // Open Graph / Facebook / Discord
        meta property="og:type"         content="website" {}
        meta property="og:url"          content="https://joeylent.dev/" {}
        meta property="og:title"        content="Joey Lent" {}
        meta property="og:description"  content=(description) {}
        meta property="og:image"        content="/banner.jpg" {}

        // Twitter
        meta property="twitter:card"        content="summary_large_image" {}
        meta property="twitter:url"         content="https://joeylent.dev/" {}
        meta property="twitter:title"       content="Joey Lent" {}
        meta property="twitter:description" content=(description) {}
        meta property="twitter:image"       content="/banner.jpg" {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meta_tags() {
        let description = "This is a description of this page.";
        let tags = meta_tags(description).into_string();

        assert_eq!(
            tags,
            format!(
                r#"<meta name="viewport" content="width=device-width,initial-scale=1.0"></meta><meta name="title" content="Joey Lent"></meta><meta name="description" content="{0}"></meta><meta property="og:type" content="website"></meta><meta property="og:url" content="https://joeylent.dev/"></meta><meta property="og:title" content="Joey Lent"></meta><meta property="og:description" content="{0}"></meta><meta property="og:image" content="/banner.jpg"></meta><meta property="twitter:card" content="summary_large_image"></meta><meta property="twitter:url" content="https://joeylent.dev/"></meta><meta property="twitter:title" content="Joey Lent"></meta><meta property="twitter:description" content="{0}"></meta><meta property="twitter:image" content="/banner.jpg"></meta>"#,
                description
            )
        );
    }
}
