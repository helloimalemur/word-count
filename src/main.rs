use crate::entities::wordcountapp::{run_calculations, WordCountApp};
use crate::entities::wordcountfile::WordCountFile;
use crate::ui::ui::WordCount;
use slint::{ComponentHandle, SharedString, Timer, TimerMode};
use std::sync::{Arc, Mutex};

mod calculations;
mod docx;
mod entities;
mod ui;
use slint::Model;

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
    let files_binding = files.clone();
    timer.start(
        TimerMode::Repeated,
        std::time::Duration::from_millis(2000),
        move || {
            run_calculations(files.clone());


            let mut guard = files_binding.lock().unwrap();
            let word_count_window_weak_handle = word_count_window.as_weak();
            let word_count_upgraded_weak_handle =
                word_count_window_weak_handle.upgrade().unwrap();
            let mut counter_value = word_count_upgraded_weak_handle.get_counter();
            let mut array = word_count_upgraded_weak_handle.get_list_of_structs();

            let mut current_row = word_count_upgraded_weak_handle
                .get_list_of_structs()
                .row_count();

            // for (ind, file) in files.lock().unwrap().iter_mut().enumerate() {
            //     // set current row as the next open place in the object array
            //     current_row = ind;
            //     let text = format!("text: {} - WordCount: {}", file.path.clone(), file.word_count);
            //     array.set_row_data(current_row, (SharedString::from(text),));
            //     word_count_upgraded_weak_handle.set_list_of_structs(array.clone());
            //     // increment counter on gui
            //     counter_value = guard.len() as i32;
            //     word_count_upgraded_weak_handle.set_counter(counter_value);
            // }


            word_count_window.window().request_redraw();
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
