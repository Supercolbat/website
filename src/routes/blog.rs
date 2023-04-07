use maud::{DOCTYPE, html, Markup, PreEscaped};
use crate::components;
use crate::utils;

#[actix_web::get("/blog")]
async fn blog() -> actix_web::Result<Markup> {
    let css = utils::compile_scss("src/sass/blog.scss");
    Ok(html! {
        (DOCTYPE)
        head {
            title { "me blog" }
            style { (PreEscaped(css)) }
        }
        header {
            h1 { "Blog" }
        }
        main {
            section.skills {
                h2 { "Fields of expertise" }
            }
        }
        (components::footer())
    })
}
