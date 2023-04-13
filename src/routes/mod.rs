//! Page routes for use in actix-web

mod index;
mod contact;
mod blog;
mod read;
mod rss;

// Publically expose routes
pub use index::*;
pub use contact::*;
pub use blog::*;
pub use read::*;
pub use rss::*;
