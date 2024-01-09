pub fn get_word_count(input: String) -> i64 {
    words_count::count(input).words as i64
}

// pub fn get_para_count(_path: String) {
//     todo!()
// }

pub fn get_nth_top_used_word(input: String, nth: usize) -> String {
    let (third, _third_size) = words_count::count_separately(input.as_str())
        .into_iter()
        .nth(nth)
        .unwrap();
    third.to_string()
}

pub fn get_unique_words(input: String) -> usize {
    let result = words_count::count_separately(input.as_str());
    result.len()
}
