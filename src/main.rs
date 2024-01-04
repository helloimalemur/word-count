use crate::entities::wordcountapp::{run_calculations, WordCountApp};
use crate::entities::wordcountfile::WordCountFile;
use crate::ui::ui::WordCount;
use slint::{ComponentHandle, Timer, TimerMode};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

mod calculations;
mod docx;
mod entities;
mod ui;

fn main() {
    let word_count_window = WordCount::new().unwrap();
    let files: Arc<Mutex<Vec<WordCountFile>>> = Arc::new(Mutex::new(Vec::<WordCountFile>::new()));

    let app = Arc::new(Mutex::new(WordCountApp::new(
        word_count_window.clone_strong(),
        files.clone(),
    )));
    app.lock()
        .unwrap()
        .config(word_count_window.clone_strong(), files.clone());

    // re-occuring
    let timer = Timer::default();
    let app_bind = app.clone();
    timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(2000),
        move || {
            run_calculations(files.clone());
        },
    );

    app.lock()
        .unwrap()
        .word_count_window
        .lock()
        .unwrap()
        .run()
        .unwrap();
}
