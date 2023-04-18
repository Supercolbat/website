//! Page routes for use in actix-web

mod index;
mod blog;
mod read;
mod rss;
mod privacy;

// Publically expose routes
pub use index::*;
pub use blog::*;
pub use read::*;
pub use rss::*;
pub use privacy::*;
