use std::sync::{Arc, Mutex};
use crate::blog::Blog;

/// Shareable state for the App
/// Different endpoints can access or modify data contained in this struct
pub struct AppState {
    pub blog: Arc<Mutex<Blog>>
}
