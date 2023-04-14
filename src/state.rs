use std::sync::{Arc, Mutex};
use crate::{blog::Blog, utils};

/// Shareable state for the App
/// Different endpoints can access or modify data contained in this struct
pub struct AppState {
    pub blog: Arc<Mutex<Blog>>,
    pub css: Arc<Mutex<PageStyle>>,
}

//
pub struct PageStyle {
    pub index: String,
    pub contact: String,
    pub blog: String,
    pub read: String,
}

impl PageStyle {
    pub fn new() -> Self {
        PageStyle {
            index: String::default(),
            contact: String::default(),
            blog: String::default(),
            read: String::default(),
        }
    }

    pub fn update_styles(&mut self) {
        self.index = utils::compile_scss("src/sass/index.scss");
        self.contact = utils::compile_scss("src/sass/contact.scss");
        self.blog = utils::compile_scss("src/sass/blog.scss");
        self.read = utils::compile_scss("src/sass/read.scss");
    }
}
