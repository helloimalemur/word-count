use crate::entities::wordcountapp::{run_calculations, WordCountApp};
use crate::entities::wordcountfile::WordCountFile;
use crate::ui::ui::WordCount;
use slint::{ComponentHandle, SharedString, SharedVector, Timer, TimerMode};
use std::sync::{Arc, Mutex};
use chrono::Local;

mod calculations;
mod docx;
mod entities;
mod ui;
use slint::Model;
use crate::docx::loader::read_docx_contents_to_string;

fn main() {
    let word_count_window = WordCount::new().unwrap();

    let mut vec = Vec::<WordCountFile>::new();
    let files: Arc<Mutex<Vec<WordCountFile>>> = Arc::new(Mutex::new(vec));

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
            let mut guard = files_bind_open.lock().unwrap();
            let word_count_upgraded_weak_handle =
                word_count_window_weak_handle_open.upgrade().unwrap();

            // let mut counter_value = word_count_upgraded_weak_handle.get_counter();
            let mut array = word_count_upgraded_weak_handle.get_list_of_structs();

            let mut bind = guard.clone();
            println!("{}", bind.len());
            for (ind, ent) in bind.iter_mut().enumerate() {
                let text = format!(
                    "text: {} - WordCount: {}",
                    ent.path.clone(),
                    calculations::calculations::get_word_count(ent.full_file_contents.to_string())
                );
                array.set_row_data(ind, (SharedString::from(text),));
            }

            word_count_upgraded_weak_handle.set_list_of_structs(array);
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
