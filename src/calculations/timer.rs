use crate::calculations;
use crate::docx::loader::read_docx_contents_to_string;
use crate::entities::wordcountfile::WordCountFile;
use crate::ui::slint_ui::WordCount;
use os_info::Type;
use slint::{Model, SharedString, Weak};
use std::rc::Rc;
use std::sync::Mutex;

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
            _ => ent.path.split('/').last().unwrap(),
        }
        .to_string();

        // get calculcations
        let white_space = calculations::counts::get_ws_count(ent.full_file_contents.to_string());
        let paragraph_count = calculations::counts::get_paragraph_count(ent.full_file_contents.to_string());
        let word_count = calculations::counts::get_word_count(ent.full_file_contents.to_string());
        let unique_word_count =
            calculations::counts::get_unique_words(ent.full_file_contents.to_string());
        let char_count = calculations::counts::get_char_count(ent.full_file_contents.to_string());
        let _char_count_no_ws = char_count - white_space;

        let one_most_used = calculations::counts::get_top_used_word(
            ent.full_file_contents.to_string().clone()
        );
        let two_most_used = calculations::counts::get_top_used_word(
            ent.full_file_contents.to_string().clone()
        );
        let third_most_used =
            calculations::counts::get_top_used_word(
                ent.full_file_contents.to_string().clone()
            );

        let mut partial_path = String::new();
        if path.len() > 30 {
            let path_bytes = &path.as_bytes()[0..30];
            partial_path = String::from_utf8(Vec::from(path_bytes)).expect("could not substring filename")
        } else {
            partial_path = path;
        }

        // create gui text output
        let text = format!(
            "{}",
            partial_path,
        );

        let text2 = format!(
            "Most Used; #1: [{}]\n Para count: {}\n Char count: {}",
            one_most_used,
            paragraph_count,
            char_count,
        );

        let text3 = format!(
            "{}",
            " "
        );

        let text4 = format!(
            "{}",
            " "
        );

        let text5 = format!(
            "Unique_words: {}\n WordCount: {}",
            unique_word_count,
            word_count,
        );





        // update gui row data
        array.set_row_data(
            ind,
            (
                SharedString::from(text),
                SharedString::from(text2),
                SharedString::from(text3),
                SharedString::from(text4),
                SharedString::from(text5),
            ),
        );
    }
}
