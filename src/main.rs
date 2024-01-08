use crate::entities::wordcountapp::{WordCountApp};
use crate::entities::wordcountfile::WordCountFile;
use crate::ui::ui::WordCount;
use slint::{ComponentHandle, Timer, TimerMode};
use std::sync::{Arc, Mutex};

mod calculations;
mod docx;
mod entities;
mod ui;
use slint::Model;

fn main() {
    let word_count_window = WordCount::new().unwrap();

    let mut files: Arc<Mutex<Vec<WordCountFile>>> = Arc::new(Mutex::new(Vec::<WordCountFile>::new()));

    let mut app = Arc::new(Mutex::new(WordCountApp::new(
        word_count_window.clone_strong(),
        files.clone(),
    )));
    app.lock()
        .unwrap()
        .config(word_count_window.clone_strong(), files.clone());

    // re-occuring
    let timer = Timer::default();
    let app_bind = app.clone();
    let files_bind_open = app_bind.lock().unwrap().files.clone();
    let word_count_window_weak_handle_open = word_count_window.as_weak();
    timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(2000),

        move || {
            calculations::timer::run_timer(files_bind_open.clone(), word_count_window_weak_handle_open.clone());
        }
    );

    app.lock()
        .unwrap()
        .word_count_window
        .lock()
        .unwrap()
        .run()
        .unwrap();
}
