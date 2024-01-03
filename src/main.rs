
use crate::entities::wordcountapp::WordCountApp;


mod ui;
mod entities;
mod docx;

fn main() {
    let app = WordCountApp::new();

    println!("{:#?}", app.files);


}

fn button_pressed() {

}
