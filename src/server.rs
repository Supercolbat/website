use actix_web::{App, HttpServer, middleware::{Logger, Compress}};
use actix_files as fs;

use crate::routes;

/// Starts the web server
/// ```rust
/// println!('hi');
/// ```
pub async fn start_server(addr: &str, port: u16) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Middleware
            .wrap(Logger::default())
            .wrap(Compress::default())

            // Routes
            .service(routes::index)
            .service(routes::blog)
            .service(fs::Files::new("/", "./public/").show_files_listing())
    })
    .bind((addr, port))?
    .run()
    .await
}

// Use actix_web::test as a unit test
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new().service(routes::index)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_blog() {
        let mut app = test::init_service(App::new().service(routes::blog)).await;
        let req = test::TestRequest::get().uri("/blog").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }
}

