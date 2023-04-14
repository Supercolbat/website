use actix_web::web;
use maud::{DOCTYPE, html, Markup, PreEscaped};
use crate::{components, state::AppState, utils};

/// Generate page for the contact page.
///
/// This page contains contact information such as E-mail and Matrix ID.
///
/// TODO: My public key is linked next to the e-mail if a user decides to encrypt their
/// communications with me.
#[actix_web::get("/contact")]
async fn contact(data: web::Data<AppState>) -> actix_web::Result<Markup> {
    let css = if cfg!(debug_assertions) {
        utils::compile_scss("src/sass/contact.scss")
    } else {
        data.css.lock().unwrap().contact.clone()
    };

    Ok(html! {
        (DOCTYPE)
        head {
            title { "Contact :: Joey Lent" }
            (components::meta_tags("Another self-proclaimed developer"))
            style { (PreEscaped(css)) }
        }
        header {
            h1 { "Contact" }
        }
        main {
            section {
                h3 { "Matrix" }
                p { "@py_:catgirl.cloud" }

                h3 { "E-mail" }
                p {
                    "supercolbat@protonmail.com " 
                    a href="#" { "(PGP)" }
                }
            }
        }
        (components::footer())
    })
}
