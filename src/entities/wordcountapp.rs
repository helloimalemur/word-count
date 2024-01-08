use crate::docx::loader::read_docx_contents_to_string;
use crate::entities::wordcountfile::WordCountFile;
use crate::ui::slint_ui::WordCount;
use native_dialog::FileDialog;
use slint::{ComponentHandle, Model, SharedString};

use std::ops::DerefMut;
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub struct WordCountApp {
    pub files: Rc<Mutex<Vec<WordCountFile>>>,
    pub word_count_window: Rc<Mutex<WordCount>>,
}

impl WordCountApp {
    pub fn new(
        word_count_window: WordCount,
        files: Rc<Mutex<Vec<WordCountFile>>>,
    ) -> WordCountApp {
        WordCountApp {
            files,
            word_count_window: Rc::new(Mutex::new(word_count_window.clone_strong())),
        }
    }

    pub fn config(&mut self, word_count_window: WordCount, files: Rc<Mutex<Vec<WordCountFile>>>) {
        let mut files_bind_open = files.clone();
        word_count_window
            // OPEN FILE BUTTON
            .on_open_file_pressed(move || {
                let mut guard = files_bind_open.lock().unwrap();

                if let Some(file) = show_open_dialog() {
                    println!("{}", file.to_str().unwrap());
                    let mut new_file = WordCountFile::new(String::from(file.to_str().unwrap()));
                    // Add WordCountFile to Vec
                    guard.push(new_file.clone());
                }
            });

        // RECALCULATE BUTTON
        let mut files_bind_re_calc = files.clone();
        word_count_window.on_re_calc_pressed(move || {
            let guard = files_bind_re_calc.clone();

            println!("Vec size; {}", guard.lock().unwrap().len());

            for file in guard.lock().unwrap().iter_mut() {
                println!("{}", file.full_file_contents);
                file.being_modified = true;
            }
        });

        // CLEAR BUTTON
        let word_count_window_weak_handle = word_count_window.as_weak();
        let files_bind_clear = files.clone();
        word_count_window.on_clear_pressed(move || {
            let guard = files_bind_clear.clone();

            let word_count_upgraded_weak_handle = word_count_window_weak_handle.upgrade().unwrap();
            let mut counter_value = word_count_upgraded_weak_handle.get_counter();
            let mut array = word_count_upgraded_weak_handle.get_list_of_structs();

            println!("Vec size; {}", guard.lock().unwrap().len());

            guard.lock().unwrap().deref_mut().clear();

            for i in 0..10 {
                array.clone().set_row_data(i, (SharedString::from(""),));
                word_count_upgraded_weak_handle.set_list_of_structs(array.clone());
                // increment counter on gui
                counter_value = guard.lock().unwrap().len() as i32;
                word_count_upgraded_weak_handle.set_counter(counter_value);
            }
        });
    }
}

fn show_open_dialog() -> Option<PathBuf> {
    FileDialog::new()
        .add_filter("docx", &["docx"])
        .show_open_single_file()
        .expect("could not open file dialog")
}
