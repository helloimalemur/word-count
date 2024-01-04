use crate::entities::wordcountfile::WordCountFile;
use crate::ui::ui::WordCount;
use chrono::Local;
use slint::{ComponentHandle, LogicalSize, Model, SharedString, Window, WindowSize};
use std::fmt::format;
use std::sync::{Arc, Mutex, MutexGuard};
pub struct WordCountApp {
    pub files: Arc<Mutex<Vec<WordCountFile>>>,
}

impl WordCountApp {
    pub fn new() -> WordCountApp {
        let word_count_window = WordCount::new().unwrap();
        let word_count_window_weak_handle = word_count_window.as_weak();
        word_count_window.on_open_file_pressed(move || {
            // word count handle
            let word_count_upgraded_weak_handle = word_count_window_weak_handle.upgrade().unwrap();
            // counter
            let mut val = word_count_upgraded_weak_handle.get_counter();

            let mut array = word_count_upgraded_weak_handle.get_list_of_structs();

            let mut current_row = word_count_upgraded_weak_handle
                .get_list_of_structs()
                .row_count();
            current_row = val as usize;
            let text = format!("text: {}", Local::now().timestamp());
            array.set_row_data(current_row, (SharedString::from(text),));
            word_count_upgraded_weak_handle.set_list_of_structs(array);

            println!("{}", current_row);

            // for i in 0..10 {
            //     let mut array = wc.get_list_of_structs();
            //     array.set_row_data(i as usize, (SharedString::from("asf"), SharedString::from("asf")))
            // }

            // increment counter
            val += 1;
            if val == 10 {
                val = 0;
            }
            word_count_upgraded_weak_handle.set_counter(val);
        });
        let ws = WindowSize::Logical(LogicalSize::new(200f32, 300f32));

        word_count_window.run().unwrap();

        WordCountApp {
            files: Arc::new(Mutex::new(vec![])),
        }
    }
    fn add_file() {}
    fn remove_file() {}
    fn get_files(&self) -> Vec<WordCountFile> {
        self.files.lock().unwrap().clone()
    }
}
