use crate::entities::wordcountfile::WordCountFile;
use crate::ui::ui::WordCount;
use chrono::Local;
use slint::{ComponentHandle, LogicalSize, Model, ModelRc, SharedString, Window, WindowSize};
use std::fmt::format;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};
use native_dialog::FileDialog;
use crate::docx::loader::read;


pub struct WordCountApp {
    pub files: Arc<Mutex<Vec<WordCountFile>>>,
    pub word_count_window: Arc<Mutex<WordCount>>,
}

impl WordCountApp {
    pub fn new() -> WordCountApp {
        let word_count_window = WordCount::new().unwrap();

        WordCountApp {
            files: Arc::new(Mutex::new(vec![])),
            word_count_window: Arc::new(Mutex::new(word_count_window.clone_strong())),
        }
    }

    pub fn config(&self) {
        let word_count_window_weak_handle = self.word_count_window.lock().unwrap().as_weak();

        let mut files_guard = self.files.lock().unwrap().clone();

        self.word_count_window
            .lock()
            .unwrap()
            .on_open_file_pressed(move || {
                let word_count_upgraded_weak_handle =
                    word_count_window_weak_handle.upgrade().unwrap();
                let mut counter_value = word_count_upgraded_weak_handle.get_counter();
                let mut array = word_count_upgraded_weak_handle.get_list_of_structs();

                // let mut current_row = word_count_upgraded_weak_handle
                //     .get_list_of_structs()
                //     .row_count();
                // current_row = counter_value as usize;
                // let text = format!("text: {}", Local::now().timestamp());
                // array.set_row_data(current_row, (SharedString::from(text),));
                // word_count_upgraded_weak_handle.set_list_of_structs(array);
                //
                // println!("{}", current_row);

                if let Some(file) = show_open_dialog() {
                    println!("{}", file.to_str().unwrap());

                    let contents = read(file.to_str().unwrap().to_string());

                    let new_file = WordCountFile {
                        path: String::from(file.to_str().unwrap()),
                        notes: "".to_string(),
                        deadline: Default::default(),
                        time_spent: 0,
                        being_modified: false,
                        word_count: 0,
                        para_count: 0,
                        unique_words: Default::default(),
                    };

                    files_guard.push(new_file);

                    list_files(files_guard.clone());


                    // increment counter
                    counter_value += 1;
                    if counter_value == 10 {
                        counter_value = 0;
                    }
                    word_count_upgraded_weak_handle.set_counter(counter_value);
                }


            });
    }

    pub fn load_file() {}
    pub fn run_calculations() {}

}

fn show_open_dialog() -> Option<PathBuf> {
    FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("PNG Image", &["docx"])
        .show_open_single_file()
        .expect("could not open file dialog")
}

pub fn list_files(vec: Vec<WordCountFile>) {
    println!("{:#?}", vec);
}
