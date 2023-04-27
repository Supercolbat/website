//! Page routes for use in actix-web

mod index;
mod blog;
mod read;
mod rss;
mod privacy;
mod error_404;

// Publically expose routes
pub use index::*;
pub use blog::*;
pub use read::*;
pub use rss::*;
pub use privacy::*;
pub use error_404::*;

