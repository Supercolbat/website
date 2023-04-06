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
