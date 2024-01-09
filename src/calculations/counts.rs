pub fn get_word_count(input: String) -> usize {
    words_count::count(input.to_lowercase()).words + get_m_dash_count(input.to_lowercase())
}

pub fn get_char_count(input: String) -> i64 {
    words_count::count(input.replace('\n', "").replace('\r', "")).characters as i64
}

pub fn get_paragraph_count(input: String) -> i64 {
    input.split("\n\n\n\n").count() as i64
}

pub fn _get_line_count(input: String) -> i64 {
    (input.split("\n\n").count()-1) as i64
}

pub fn get_ws_count(input: String) -> i64 {
    words_count::count(input).whitespaces as i64
}

// pub fn get_para_count(_path: String) {
//     todo!()
// }

pub fn get_top_used_word(input: String) -> String {
    let binding = input.to_lowercase();
    let (nth_string, nth_size) = words_count::count_separately(binding.as_str())
        .into_iter()
        .max_by_key(|entry | entry.1)
        .unwrap();

    nth_string.to_string()
}

pub fn get_m_dash_count(input: String) -> usize {
    let mut out: usize = 0;
    // println!("{}", input.find("\u{2014}").unwrap());
    // let x = input.find("\u{2014}");
    // if x.is_some() {
    //     out = x.unwrap() / 214;
    // }
    out
    // 0
}

pub fn get_unique_words(input: String) -> usize {
    let binding = input.to_lowercase();
    let result = words_count::count_separately(binding.as_str());
    result.len()
}
