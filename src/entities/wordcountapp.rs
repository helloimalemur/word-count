use crate::entities::wordcountfile::WordCountFile;
use crate::ui::ui::WordCount;
use chrono::Local;
use slint::{ComponentHandle, LogicalSize, Model, ModelRc, SharedString, Window, WindowSize};
use std::fmt::format;
use std::sync::{Arc, Mutex, MutexGuard};
pub struct WordCountApp {
    pub files: Arc<Mutex<Vec<WordCountFile>>>,
    pub word_count_window: Arc<Mutex<WordCount>>,
}

impl WordCountApp {
    pub fn new() -> WordCountApp {
        let word_count_window = WordCount::new().unwrap();

        // let word_count_window_weak_handle = word_count_window.as_weak();
        // word_count_window.on_open_file_pressed(move || {
        //     let word_count_upgraded_weak_handle = word_count_window_weak_handle.upgrade().unwrap();
        //     open_file(&word_count_upgraded_weak_handle);
        // });

        // word_count_window.run().unwrap();

        WordCountApp {
            files: Arc::new(Mutex::new(vec![])),
            word_count_window: Arc::new(Mutex::new(word_count_window.clone_strong())),
        }
    }
    pub fn config(&self) {
        let word_count_window_weak_handle = self.word_count_window.lock().unwrap().as_weak();
        self.word_count_window.lock().unwrap().on_open_file_pressed(move || {
            let word_count_upgraded_weak_handle = word_count_window_weak_handle.upgrade().unwrap();
            open_file(&word_count_upgraded_weak_handle);
        });
        self.word_count_window.lock().unwrap().run().unwrap();
    }

    fn remove_file() {}
    fn get_files(&self) -> Vec<WordCountFile> {
        self.files.lock().unwrap().clone()
    }
}

pub fn open_file(word_count_upgraded_weak_handle: &WordCount) {
    let mut counter_value = word_count_upgraded_weak_handle.get_counter();
    let mut array = word_count_upgraded_weak_handle.get_list_of_structs();

    let mut current_row = word_count_upgraded_weak_handle
        .get_list_of_structs()
        .row_count();
    current_row = counter_value as usize;
    let text = format!("text: {}", Local::now().timestamp());
    array.set_row_data(current_row, (SharedString::from(text),));
    word_count_upgraded_weak_handle.set_list_of_structs(array);

    println!("{}", current_row);

    // increment counter
    counter_value += 1;
    if counter_value == 10 {
        counter_value = 0;
    }
    word_count_upgraded_weak_handle.set_counter(counter_value);
}
