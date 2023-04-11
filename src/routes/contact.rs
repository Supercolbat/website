use maud::{DOCTYPE, html, Markup, PreEscaped};
use crate::{components, utils};

#[actix_web::get("/contact")]
async fn contact() -> actix_web::Result<Markup> {
    let css = utils::compile_scss("src/sass/contact.scss");
    Ok(html! {
        (DOCTYPE)
        head {
            title { "Joey Lent" }
            (components::meta_tags("Another self-proclaimed developer"))
            style { (PreEscaped(css)) }
        }
        header {
            h1 { "Contact" }
        }
        main {
            section {
                h3 { "Email" }
                p {
                    "supercolbat@protonmail.com " 
                    a href="#" { "(PGP)" }
                }
                h3 { "Matrix" }
                p { "@py_:catgirl.cloud" }
            }
        }
        (components::footer())
    })
}
