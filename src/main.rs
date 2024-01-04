use std::thread;
use std::time::Duration;
use slint::ComponentHandle;
use crate::entities::wordcountapp::WordCountApp;

mod docx;
mod entities;
mod ui;

fn main() {
    let app = WordCountApp::new();
    app.config();
    app.word_count_window.lock().unwrap().run().unwrap();
}
