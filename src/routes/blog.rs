use maud::{DOCTYPE, html, Markup};

#[actix_web::get("/blog")]
async fn blog() -> actix_web::Result<Markup> {
    Ok(html! {
        (DOCTYPE)
        head {
            title { "me blog" }
        }
        h1 { "hello wolrd" }
    })
}
