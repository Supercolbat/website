use std::sync::{Arc, Mutex};
use crate::blog::Blog;

pub struct AppState {
    pub blog: Arc<Mutex<Blog>>
}
