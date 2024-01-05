use crate::docx::loader::read_docx_contents_to_string;
use crate::entities::wordcountfile::WordCountFile;
use crate::ui::ui::WordCount;
use chrono::Local;
use native_dialog::FileDialog;
use slint::{ComponentHandle, LogicalSize, Model, ModelRc, SharedString, Timer, TimerMode, Weak, Window, WindowSize};
use std::fmt::format;
use std::ops::DerefMut;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct WordCountApp {
    pub files: Arc<Mutex<Vec<WordCountFile>>>,
    pub word_count_window: Arc<Mutex<WordCount>>,
}

impl WordCountApp {
    pub fn new(
        word_count_window: WordCount,
        files: Arc<Mutex<Vec<WordCountFile>>>,
    ) -> WordCountApp {
        WordCountApp {
            files,
            word_count_window: Arc::new(Mutex::new(word_count_window.clone_strong())),
        }
    }

    pub fn config(&mut self, word_count_window: WordCount, files: Arc<Mutex<Vec<WordCountFile>>>) {

        let word_count_window_weak_handle_open = word_count_window.as_weak();
        let mut files_bind_open = files.clone();
        word_count_window
            // OPEN FILE BUTTON
            .on_open_file_pressed(move || {
                let mut guard = files_bind_open.lock().unwrap();

                let word_count_upgraded_weak_handle =
                    word_count_window_weak_handle_open.upgrade().unwrap();
                let mut counter_value = word_count_upgraded_weak_handle.get_counter();
                let mut array = word_count_upgraded_weak_handle.get_list_of_structs();

                if let Some(file) = show_open_dialog() {
                    println!("{}", file.to_str().unwrap());

                    let mut new_file = WordCountFile {
                        path: String::from(file.to_str().unwrap()),
                        notes: "".to_string(),
                        deadline: Default::default(),
                        time_spent: 0,
                        being_modified: false,
                        word_count: 0,
                        para_count: 0,
                        unique_words: Default::default(),
                        full_file_contents: read_docx_contents_to_string(file.to_str().unwrap().to_string()),
                    };

                    // Add WordCountFile to Vec
                    guard.push(new_file.clone());

                    // Update Gui "Table"
                    let mut current_row = word_count_upgraded_weak_handle
                        .get_list_of_structs()
                        .row_count();
                    // set current row as the next open place in the object array
                    current_row = guard.len() + 1usize;
                    let text = format!("text: {} - WordCount: {}", new_file.path.clone(), new_file.word_count);
                    array.set_row_data(current_row, (SharedString::from(text),));
                    word_count_upgraded_weak_handle.set_list_of_structs(array);
                    // increment counter on gui
                    counter_value = guard.len() as i32;
                    word_count_upgraded_weak_handle.set_counter(counter_value);

                    // Debugging
                    // list_files(guard.clone());
                }
            });

        let word_count_window_weak_handle_re_calc = word_count_window.as_weak();
        let mut files_bind_re_calc = files.clone();
        word_count_window.on_re_calc_pressed(move || {
            let guard = files_bind_re_calc.clone();

            let word_count_upgraded_weak_handle = word_count_window_weak_handle_re_calc.upgrade().unwrap();
            let mut counter_value = word_count_upgraded_weak_handle.get_counter();
            let mut array = word_count_upgraded_weak_handle.get_list_of_structs();

            println!("Vec size; {}", guard.lock().unwrap().len());


            for (ind,file) in guard.lock().unwrap().iter_mut().enumerate() {
                println!("{}", file.full_file_contents);
                file.being_modified = true;
            }
        });



        let word_count_window_weak_handle = word_count_window.as_weak();
        let files_bind_clear = files.clone();
        word_count_window.on_clear_pressed(move || {
            let guard = files_bind_clear.clone();

            let word_count_upgraded_weak_handle = word_count_window_weak_handle.upgrade().unwrap();
            let mut counter_value = word_count_upgraded_weak_handle.get_counter();
            let mut array = word_count_upgraded_weak_handle.get_list_of_structs();

            println!("Vec size; {}", guard.lock().unwrap().len());

            guard.lock().unwrap().deref_mut().clear();

            // for (ind,file) in guard.lock().unwrap().iter().enumerate() {
            //     println!("{}", file.full_file_contents);
            // }

            for i in 0..10 {
                array.clone().set_row_data(i, (SharedString::from(""),));
                word_count_upgraded_weak_handle.set_list_of_structs(array.clone());
                // increment counter on gui
                counter_value = guard.lock().unwrap().len() as i32;
                word_count_upgraded_weak_handle.set_counter(counter_value);
            }
        });
    }

    pub fn load_file() {}
}


pub fn run_calculations(files: Arc<Mutex<Vec<WordCountFile>>>) {
    println!("Vec size: {}", files.lock().unwrap().len());
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
