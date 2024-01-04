use crate::entities::wordcountapp::WordCountApp;

mod docx;
mod entities;
mod ui;

fn main() {
    let app = WordCountApp::new();
    app.config();
}
