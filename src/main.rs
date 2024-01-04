use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use slint::{ComponentHandle, Timer, TimerMode};
use crate::entities::wordcountapp::WordCountApp;

mod docx;
mod entities;
mod ui;
mod calculations;

fn main() {
    let app = Arc::new(Mutex::new(WordCountApp::new()));
    app.lock().unwrap().config();

    let timer = Timer::default();
    let moved_app = app.clone();
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(2000), move || {
        // moved_app.lock().unwrap().run_calculations();
        println!("{}", 1);
    });

    app.lock().unwrap().word_count_window.lock().unwrap().run().unwrap();
}
