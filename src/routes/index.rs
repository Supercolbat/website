use maud::{DOCTYPE, html, Markup, PreEscaped};
use crate::{components::{self, Icon}, utils};

/// Generate page for the blog listing.
///
/// TODO: This page is static and can likely be built at compile-time.
#[actix_web::get("/")]
async fn index() -> actix_web::Result<Markup> {
    let css = utils::compile_scss("src/sass/index.scss");
    Ok(html! {
        (DOCTYPE)
        head {
            title { "Joey Lent" }
            (components::meta_tags("Another self-proclaimed developer"))
            style { (PreEscaped(css)) }
        }
        div {
            header {
                h1 { "Joey Lent" }
                span { "Another self-proclaimed developer" }
            }
            main {
                section.about {
                    h2 { "About me" }
                    p { "Hey, I'm Joey. I'd describe myself right now, but I Lorem ipsum dolor sit amet, officia excepteur ex fugiat reprehenderit enim labore culpa sint ad nisi Lorem pariatur mollit ex esse exercitation amet." }
                }
                section.skills {
                    h2 { "Fields of expertise" }
                    // Wrapper for the cards
                    div.cards {
                        (components::card(Icon::Rust,   "Rust",   "Absolutely absorbed in this language. Even this website is made with Rust."))
                        (components::card(Icon::Nim,    "Nim",    "A very expressive language that shines in certain scenarios."))
                        (components::card(Icon::Python, "Python", "A very fitting language for scripting."))
                        (components::card(Icon::Dart,   "Dart",   "Dart and Flutter are my go-to tools for hackathons."))
                        (components::card(Icon::Figma,  "Figma",  "I use Figma regularly to design a variety of applications."))
                        (components::card(Icon::Vim,    "Vim",    "90% of my workflow exists within a Neovim environment."))
                    }
                }
            }
        }
        (components::footer())
    })
}
