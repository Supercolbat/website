use actix_web::{
    App,
    HttpServer,
    middleware::{Logger, Compress, NormalizePath, TrailingSlash},
    web, dev::Server
};
use actix_files as fs;

use std::net::SocketAddrV4;
use std::sync::{Arc, Mutex};

use crate::{routes, state::{AppState, PageStyle}, blog::Blog};

/// Creates a web server object that can be started later
pub fn create_server(addr: SocketAddrV4, blog: Arc<Mutex<Blog>>, css: Arc<Mutex<PageStyle>>) -> Server {
    HttpServer::new(move || {
        let blog = blog.clone();
        let css = css.clone();
        App::new()
            // App state
            .app_data(web::Data::new(AppState { blog, css }))

            // Middleware
            .wrap(Logger::new(r#"[%s] "%r" took %Dms"#))
            .wrap(Compress::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))

            // Routes
            .service(routes::index)
            .service(routes::blog)
            .service(routes::read)
            .service(routes::privacy)

            // Obligatory easter egg
            .service(web::redirect("/admin", "https://piped.projectsegfau.lt/watch?v=dQw4w9WgXcQ"))

            // RSS Feed
            .service(routes::rss)

            // Allow visitors to view files in the public directory
            // Files in the public directory include robots.txt, favicon.ico, and others.
            .service(fs::Files::new("/", "./public/"))

            .default_service(
                 web::route().to(routes::error_404)
            )
    })
    .bind(addr).unwrap()
    .run()
}

