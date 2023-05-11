use actix_web::{web, HttpResponse, http::StatusCode};
use maud::{DOCTYPE, html, PreEscaped};
use crate::{components, state::AppState, utils};

/// Generate page for the blog listing.
///
/// TODO: This page is static and can likely be built at compile-time.
pub async fn error_404(data: web::Data<AppState>) -> actix_web::Result<HttpResponse> {
    let css = if cfg!(debug_assertions) {
        utils::compile_scss("src/sass/error.scss")
    } else {
        data.css.lock().unwrap().error.clone()
    };

    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
        .body(html! {
            (DOCTYPE)
            head {
                title { "Joey Lent" }
                style { (PreEscaped(css)) }
            }
            div {
                header {
                    h1 { "404" }
                    span { "Page not found" }
                }
            }
            (components::footer())
        }.into_string())
    )
}
