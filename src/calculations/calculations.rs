use words_count::WordsCount;

pub fn get_word_count(input: String) -> i64 {
    words_count::count(input).words as i64
}

pub fn get_para_count(path: String) {
    todo!()
}

pub fn get_unique_words(path: String) -> usize {
    let result = words_count::count_separately("apple banana apple");
    result.len()
}
