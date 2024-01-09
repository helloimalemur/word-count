pub fn get_word_count(input: String) -> i64 {
    words_count::count(input).words as i64
}

pub fn get_char_count(input: String) -> i64 {
    words_count::count(input.replace('\n', "").replace('\r', "")).characters as i64
}

pub fn get_cjk_count(input: String) -> i64 {
    input.split("\n\n").count() as i64
}

pub fn get_ws_count(input: String) -> i64 {
    words_count::count(input).whitespaces as i64
}

// pub fn get_para_count(_path: String) {
//     todo!()
// }

pub fn get_nth_top_used_word(input: String, nth: usize) -> (String, usize) {
    let (nth_string, nth_count) = words_count::count_separately(input.as_str())
        .into_iter()
        .nth(nth)
        .unwrap();
    (nth_string.to_string(), nth_count)
}

pub fn get_unique_words(input: String) -> usize {
    let result = words_count::count_separately(input.as_str());
    result.len()
}
