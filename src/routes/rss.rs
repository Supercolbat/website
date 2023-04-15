use actix_web::{web, HttpResponse};
use crate::state::AppState;

/// Generate RSS feed.
///
/// Blog posts, named articles in the source, are read from cache and listed. If a post has an
/// empty publish date, it is considered unpublished and not shown. They can still be access
/// directly, however.
///
/// TODO: Additionally, blog posts are sorted chronologically, from latest to oldest.
#[actix_web::get("/rss")]
async fn rss(data: web::Data<AppState>) -> HttpResponse {
    let blog = data.blog.lock().unwrap();

    HttpResponse::Ok()
        .content_type("application/rss+xml; charset=utf-8")
        .body(blog.rss.clone())
}
