use maud::{DOCTYPE, html, Markup, PreEscaped};
use grass::OutputStyle;
use crate::{components, icons::Icon};

#[actix_web::get("/")]
async fn index() -> actix_web::Result<Markup> {
    let css = match grass::from_path("src/sass/index.scss", &grass::Options::default().style(OutputStyle::Compressed)) {
        Ok(css) => css,
        Err(err) => {
            // Report error in console and optionally return a debug stylesheet with the error
            let kind = err.kind();
            eprintln!("{:?}", kind);

            // Debug build
            if cfg!(debug_assertions) {
                let error = format!("{:?}", kind).replace("'", "\\'");
                // CSS magic to display hide page content and display the error message
                // ```css
                // body { display: none; }
                // html::before { content: 'error message here...' }
                // ```
                String::from(format!("body{{display:none}}html::before{{content:'{}'}}", error))
            }

            // Release build
            else {
                String::new()
            }
        }
    };

    Ok(html! {
        (DOCTYPE)
        head {
            title { "me websit" }
            style { (PreEscaped(css)) }
        }
        header {
            h1 { "Joey Lent" }
            span { "Sorry, I really like minimalism" }
        }
        main {
            section.skills {
                h2 { "Fields of expertise" }
                // Wrapper for the cards
                div.cards {
                    (components::card(Icon::Rust,   "Rust",   "Absolutely absorbed in this language. Even this website is made with Rust."))
                    (components::card(Icon::Nim,    "Nim",    ""))
                    (components::card(Icon::Python, "Python", ""))
                    (components::card(Icon::Dart,   "Dart",   "Dart and Flutter are my go-to tools for hackathons."))
                    (components::card(Icon::Figma,  "Figma",  "I use Figma regularly to design a variety of applications."))
                    (components::card(Icon::Vim,    "Vim",    "90% of my workflow exists within a Neovim environment."))
                }
            }
        }
        (components::footer())
    })
}
