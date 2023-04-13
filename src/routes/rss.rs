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

    // Load articles in order
    let mut articles: Vec<_> = (&blog).articles.iter().collect();
    articles.sort_by_key(|key| &key.1.publish_date);
    articles.reverse();

    // Set up XML writer

    HttpResponse::Ok()
        .content_type("application/rss+xml; charset=utf-8")
        .body(blog.rss.clone())
}
