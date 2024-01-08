use std::rc::Rc;
use crate::calculations;
use crate::docx::loader::read_docx_contents_to_string;
use crate::entities::wordcountfile::WordCountFile;
use crate::ui::slint_ui::WordCount;
use slint::{Model, SharedString, Weak};
use std::sync::{Mutex};
use os_info::Type;

pub fn run_timer(
    files_bind_open: Rc<Mutex<Vec<WordCountFile>>>,
    word_count_window_weak_handle_open: Weak<WordCount>,
) {
    let guard = files_bind_open.lock().unwrap();
    let word_count_upgraded_weak_handle = word_count_window_weak_handle_open.upgrade().unwrap();

    // let mut counter_value = word_count_upgraded_weak_handle.get_counter();
    let array = word_count_upgraded_weak_handle.get_list_of_structs();

    let mut bind = guard.clone();

    // for each file in the Vec
    for (ind, ent) in bind.iter_mut().enumerate() {
        // reload file contents
        ent.full_file_contents = read_docx_contents_to_string(ent.path.to_string());

        // get file name only
        let info = os_info::get();
        let path: String = match info.os_type() {
            Type::Windows => ent.path.split('\\').last().unwrap(),
            _ => ent.path.split('/').last().unwrap()
        }.to_string();

        // create gui text output
        let text = format!(
            "{} - WordCount: {}",
            path,
            calculations::counts::get_word_count(ent.full_file_contents.to_string())
        );

        // update gui row data
        array.set_row_data(ind, (SharedString::from(text),));
    }

    // push new gui data to gui state
    word_count_upgraded_weak_handle.set_list_of_structs(array);
}
