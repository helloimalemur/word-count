
pub fn get_word_count(input: String) -> i64 {
    words_count::count(input).words as i64
}

// pub fn get_para_count(_path: String) {
//     todo!()
// }

pub fn get_unique_words(input: String) -> usize {
    let result = words_count::count_separately(input.as_str());
    result.len()
}
