use actix_web::web;
use maud::{DOCTYPE, html, Markup, PreEscaped};
use crate::{components::{self, Icon}, state::AppState, utils};

/// Generate page for the blog listing.
///
/// TODO: This page is static and can likely be built at compile-time.
#[actix_web::get("/")]
async fn index(data: web::Data<AppState>) -> actix_web::Result<Markup> {
    let css = if cfg!(debug_assertions) {
        utils::compile_scss("src/sass/index.scss")
    } else {
        data.css.lock().unwrap().index.clone()
    };

    Ok(html! {
        (DOCTYPE)
        head {
            title { "Joey Lent" }
            (components::meta_tags("Joey Lent", "Another self-proclaimed developer", ""))
            style { (PreEscaped(css)) }
        }
        div {
            header {
                h1 { "Joey Lent" }
                span { "I am an upcoming front-end developer with an obsession for almost-minimalism." }
            }
            main {
                section {
                    h2 { "Fields of expertise" }
                    // Wrapper for the cards
                    div.cards {
                        (components::card(Icon::Rust,   "Rust",   "I'm quickly incorporating Rust into more and more projects."))
                        (components::card(Icon::Nim,    "Nim",    "A very expressive language that shines in certain scenarios."))
                        (components::card(Icon::Python, "Python", "I use Python often when needing to write quick scripts."))
                        (components::card(Icon::Dart,   "Dart",   "Dart and Flutter are my go-to tools for hackathons."))
                        (components::card(Icon::Figma,  "Figma",  "I use Figma regularly to design a variety of applications."))
                        (components::card(Icon::Vim,    "Vim",    "90% of my workflow exists within a Neovim environment."))
                    }
                }
                section {
                    h2 { "Contact" }

                    h3 { "Matrix" }
                    p { "@py_:catgirl.cloud" }

                    h3 { "E-mail" }
                    p {
                        "supercolbat@protonmail.com " 
                        a href="#" { "(PGP)" }
                    }
                }
            }
        }
        (components::footer())
    })
}
