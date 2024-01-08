use std::sync::{Arc, Mutex};
use slint::{Model, SharedString, Weak};
use crate::calculations;
use crate::docx::loader::read_docx_contents_to_string;
use crate::entities::wordcountfile::WordCountFile;
use crate::ui::ui::WordCount;

pub fn run_timer(files_bind_open: Arc<Mutex<Vec<WordCountFile>>>, word_count_window_weak_handle_open: Weak<WordCount>) {
    let mut guard = files_bind_open.lock().unwrap();
    let word_count_upgraded_weak_handle =
        word_count_window_weak_handle_open.upgrade().unwrap();

    // let mut counter_value = word_count_upgraded_weak_handle.get_counter();
    let mut array = word_count_upgraded_weak_handle.get_list_of_structs();

    let mut bind = guard.clone();
    println!("{}", bind.len());
    for (ind, ent) in bind.iter_mut().enumerate() {
        // reload file contents
        ent.full_file_contents = read_docx_contents_to_string(ent.path.to_string());

        let text = format!(
            "text: {} - WordCount: {}",
            ent.path.clone(),
            calculations::calculations::get_word_count(ent.full_file_contents.to_string())
        );
        array.set_row_data(ind, (SharedString::from(text),));
    }

    word_count_upgraded_weak_handle.set_list_of_structs(array);
}
