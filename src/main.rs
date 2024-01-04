use std::thread;
use std::time::Duration;
use slint::{ComponentHandle, Timer, TimerMode};
use crate::entities::wordcountapp::WordCountApp;

mod docx;
mod entities;
mod ui;
mod calculations;

fn main() {
    let app = WordCountApp::new();
    app.config();

    let timer = Timer::default();
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(200), move || {
        println!("{}", 0);
    });
    app.word_count_window.lock().unwrap().run().unwrap();
}
