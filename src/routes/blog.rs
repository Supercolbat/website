use actix_web::web;
use maud::{DOCTYPE, html, Markup, PreEscaped};
use crate::{components, state::AppState, utils};

/// Generate page for the blog listing.
///
/// Blog posts, named articles in the source, are read from cache and listed. If a post has an
/// empty publish date, it is considered unpublished and not shown. They can still be access
/// directly, however.
///
/// TODO: Additionally, blog posts are sorted chronologically, from latest to oldest.
#[actix_web::get("/blog")]
async fn blog(data: web::Data<AppState>) -> actix_web::Result<Markup> {
    let blog = data.blog.lock().unwrap();

    let css = utils::compile_scss("src/sass/blog.scss");

    let mut articles: Vec<_> = (&blog).articles.iter().collect();
    articles.sort_by_key(|key| &key.1.publish_date);
    articles.reverse();

    Ok(html! {
        (DOCTYPE)
        head {
            title { "Blog :: Joey Lent" }
            (components::meta_tags("Another self-proclaimed developer"))
            style { (PreEscaped(css)) }
        }
        div {
            header {
                h1 { "Blog" }
            }
            main {
                section {
                    ul {
                        @for article in articles {
                            @let (slug, post) = article;
                            @if ! &post.publish_date.is_empty() {
                                li {
                                    // Date is in the format of "YYYY-MM-DD HH:MM"
                                    // Diplay date without the time in case multiple posts are made
                                    // in one day
                                    "[" p { (post.publish_date.split_whitespace().next().unwrap()) } "] "

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
