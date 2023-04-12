use actix_web::{
    App,
    HttpServer,
    middleware::{Logger, Compress, NormalizePath, TrailingSlash},
    web, dev::Server
};
use actix_files as fs;

use std::net::SocketAddrV4;
use std::sync::{Arc, Mutex};

use crate::{routes, blog::Blog, state::AppState};

/// Creates a web server object that can be started later
pub fn create_server(addr: SocketAddrV4, blog: Arc<Mutex<Blog>>) -> Server {
    HttpServer::new(move || {
        let blog = blog.clone();
        App::new()
            // App state
            .app_data(web::Data::new(AppState { blog }))

            // Middleware
            .wrap(Logger::default())
            .wrap(Compress::default())
            .wrap(NormalizePath::new(TrailingSlash::Trim))

            // Routes
            .service(routes::index)
            .service(routes::contact)
            .service(routes::blog)
            .service(routes::read)

            // RSS Feed
            // .service(routes::rss)

            // Allow visitors to view files in the public directory
            // Files in the public directory include robots.txt, favicon.ico, and others.
            .service(fs::Files::new("/", "./public/"))
    })
    .bind(addr).unwrap()
    .run()
}

// Use actix_web::test as a unit test
#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::blog::Article;

    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_index_get() {
        let mut app = test::init_service(App::new().service(routes::index)).await;

        // Can the endpoint be reached?
        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_contact_get() {
        let mut app = test::init_service(App::new().service(routes::contact)).await;

        // Can the endpoint be reached?
        let req = test::TestRequest::get().uri("/contact").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_blog_get() {
        // Create the article(s)
        let mut articles = HashMap::new();
        articles.insert(String::from("test"), Article {
            title: String::default(),
            description: String::default(),
            publish_date: String::default(),
            content: String::default(),
            words: 0,
            minutes: 0,
        });

        // Create the blog
        let blog = Arc::new(Mutex::new(Blog { articles }));

        // Create the app state
        let state = AppState { blog: Arc::clone(&blog) };

        // Initialize the app
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(state))
                .wrap(Logger::default())
                .wrap(Compress::default())
                .wrap(NormalizePath::new(TrailingSlash::Trim))
                .service(routes::blog)
        ).await;

        // Can the endpoint be reached?
        let req = test::TestRequest::get().uri("/blog").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_read_get() {
        // Create the article(s)
        let mut articles = HashMap::new();
        articles.insert(String::from("test"), Article {
            title: String::default(),
            description: String::default(),
            publish_date: String::default(),
            content: String::default(),
            words: 0,
            minutes: 0,
        });

        // Create the blog
        let blog = Arc::new(Mutex::new(Blog { articles }));

        // Create the app state
        let state = AppState { blog: Arc::clone(&blog) };

        // Initialize the app
        let mut app = test::init_service(
            App::new()
                .app_data(web::Data::new(state))
                .wrap(Logger::default())
                .wrap(Compress::default())
                .wrap(NormalizePath::new(TrailingSlash::Trim))
                .service(routes::blog)
        ).await;
        
        // Can the endpoint be reached?
        // Use an existing blog post
        let req = test::TestRequest::get().uri("/blog/test").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        // Does an non-existing endpoint return an error?
        // Because of how slugs are generated, this should be an impossible endpoint
        let req = test::TestRequest::get().uri("/blog/.md").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }
}

