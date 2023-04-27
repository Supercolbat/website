use actix_web::web;
use maud::{DOCTYPE, html, Markup, PreEscaped};
use crate::{components, state::AppState, utils};

/// Generate page for the contact page.
///
/// This page contains contact information such as E-mail and Matrix ID.
///
/// TODO: My public key is linked next to the e-mail if a user decides to encrypt their
/// communications with me.
#[actix_web::get("/privacy")]
async fn privacy(data: web::Data<AppState>) -> actix_web::Result<Markup> {
    let css = if cfg!(debug_assertions) {
        utils::compile_scss("src/sass/privacy.scss")
    } else {
        data.css.lock().unwrap().privacy.clone()
    };

    Ok(html! {
        (DOCTYPE)
        head {
            title { "Privacy policy :: Joey Lent" }
            (components::meta_tags("Another self-proclaimed developer"))
            style { (PreEscaped(css)) }
        }
        header {
            h1 { "Privacy policy" }
        }
        main {
            section {
                p { b { "Yes, I keep logs, but I don't log anything personally identifiable." } }
                p { "The only information being collected is the following." }
                ol {
                    li { "Time of the request" }
                    li { "The page being visited" }
                    li { "How long it took the server to reply" }
                    li { "The status code" }
                }
                p { "I use the word " em { "\"you\"" } " very vaguely here since I have no way of knowing who you are." }
                p { "The following is an example of what the logs look like." }
                pre {
r#"[2023-04-15T15:04:32Z INFO  actix_web::middleware::logger] [200] "GET /blog HTTP/1.0" took 11.196175ms
[2023-04-15T15:04:42Z INFO  actix_web::middleware::logger] [200] "GET /blog/readme HTTP/1.0" took 12.810040ms
[2023-04-15T15:05:10Z INFO  actix_web::middleware::logger] [200] "GET /rss HTTP/1.0" took 2.167296ms
[2023-04-15T15:05:15Z INFO  actix_web::middleware::logger] [200] "GET /rss HTTP/1.0" took 2.512870ms
[2023-04-15T15:05:38Z INFO  actix_web::middleware::logger] [200] "GET / HTTP/1.0" took 14.726532ms
[2023-04-15T15:05:42Z INFO  actix_web::middleware::logger] [200] "GET /contact HTTP/1.0" took 10.689164ms"#
                }
                p { "Logs will not be directly shared with anybody, but I may disclose a summary of the logs." }
                p { "An example of what that may look like with the logs above may be." }
                pre { "Someone read my readme blog post on April 15th." }
            }
        }
        (components::footer())
    })
}
