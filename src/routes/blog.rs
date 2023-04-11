use actix_web::web;
use maud::{DOCTYPE, html, Markup, PreEscaped};
use crate::{components, state::AppState, utils};

#[actix_web::get("/blog")]
async fn blog(data: web::Data<AppState>) -> actix_web::Result<Markup> {
    let blog = data.blog.lock().unwrap();

    let css = utils::compile_scss("src/sass/blog.scss");

    Ok(html! {
        (DOCTYPE)
        head {
            title { "Joey Lent :: Blog" }
            (components::meta_tags("Another self-proclaimed developer"))
            style { (PreEscaped(css)) }
        }
        div {
            header {
                h1 { "Blog" }
            }
            main {
                p { "TODO: These are not sorted chronologically." }
                section {
                    ul {
                        @for article in &blog.articles {
                            @let (slug, post) = article;
                            @if ! &post.publish_date.is_empty() {
                                li {
                                    "["
                                    p { (post.publish_date) }
                                    "] "
                                    a href=(format!("/blog/{}", slug)) { (post.title) }
                                }
                            }
                        }
                    }
                }
            }
        }
        (components::footer())
    })
}
