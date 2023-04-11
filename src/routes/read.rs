use actix_web::web;
use maud::{DOCTYPE, html, Markup, PreEscaped};
use crate::{components, state::AppState, utils};

#[actix_web::get("/blog/{slug}")]
async fn read(data: web::Data<AppState>, slug: web::Path<(String,)>) -> actix_web::Result<Markup> {
    let blog = data.blog.lock().unwrap();

    let content = &blog.get(slug.into_inner().0);
    if content.is_none() {
        return Ok(html! {

        })
    }
    let article = content.unwrap();
    let css = utils::compile_scss("src/sass/read.scss");

    Ok(html! {
        (DOCTYPE)
        head {
            title { "Joey Lent :: " (&article.title) }
            (components::meta_tags(&article.description))
            style { (PreEscaped(css)) }
        }
        div {
            header {
                h1 { (&article.title) }
                span { (&article.description) }
            }
            main {
                section.metadata {
                    p { "Published on: " (&article.publish_date) }
                    hr {}
                    p {
                        (&article.words.to_string())
                        " words / " 
                        (&article.minutes)
                        @if &article.minutes == &1 { " minute" }
                        @else { " minutes" }
                    }
                }
                section.article {
                    (PreEscaped(&article.content))
                }
            }
        }
        (components::footer())
    })
}
