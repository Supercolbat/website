use actix_web::web;
use maud::{DOCTYPE, html, Markup, PreEscaped};
use crate::{components, state::AppState, utils};

/// Generate page for the blog listing.
///
/// Reads an article from cache and generates a page with the information.
#[actix_web::get("/blog/{slug}")]
async fn read(data: web::Data<AppState>, slug: web::Path<(String,)>) -> actix_web::Result<Markup> {
    let blog = data.blog.lock().unwrap();

    let content = &blog.get(slug.into_inner().0);
    let article = content.unwrap();

    let css = if cfg!(debug_assertions) {
        utils::compile_scss("src/sass/read.scss")
    } else {
        data.css.lock().unwrap().read.clone()
    };

    Ok(html! {
        (DOCTYPE)
        head {
            title { (&article.title) " :: Joey Lent"}
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
                    @if (&article).publish_date.is_empty() {
                        p { b { "Previewing article" } }
                    }
                    @else {
                        p { "Published on: " (&article.publish_date) }
                    }

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
