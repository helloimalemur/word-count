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
        let cjk = calculations::counts::get_cjk_count(ent.full_file_contents.to_string());
        let word_count = calculations::counts::get_word_count(ent.full_file_contents.to_string());
        let unique_word_count =
            calculations::counts::get_unique_words(ent.full_file_contents.to_string());
        let char_count = calculations::counts::get_char_count(ent.full_file_contents.to_string());
        let char_count_no_ws = char_count - white_space;

        let (one_most_used, one_most_used_occurances) = calculations::counts::get_nth_top_used_word(
            ent.full_file_contents.to_string().clone(),
            1,
        );
        let (two_most_used, two_most_used_occurances) = calculations::counts::get_nth_top_used_word(
            ent.full_file_contents.to_string().clone(),
            2,
        );
        let (third_most_used, third_most_used_occurances) =
            calculations::counts::get_nth_top_used_word(
                ent.full_file_contents.to_string().clone(),
                3,
            );

        // create gui text output
        let text = format!(
            "{} - Most Used; #1: [{}:{}], CJK count: {}, Char count: {}, Unique_words: {}, WordCount: {}",
            path,
            one_most_used,
            one_most_used_occurances,
            cjk,
            char_count,
            unique_word_count,
            word_count,
        );

        // update gui row data
        array.set_row_data(
            ind,
            (
                SharedString::from(text),
                SharedString::from(""),
                SharedString::from(""),
                SharedString::from(""),
                SharedString::from(""),
            ),
        );
    }
}
