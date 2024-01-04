use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WordCountFile {
    pub(crate) path: String,
    pub(crate) notes: String,
    pub(crate) deadline: DateTime<Utc>,
    pub(crate) time_spent: i128,
    pub(crate) being_modified: bool,
    pub(crate) word_count: i128,
    pub(crate) para_count: i32,
    pub(crate) unique_words: HashMap<String, i32>,
}

impl WordCountFile {
    fn new(path: String) -> WordCountFile {
        WordCountFile {
            path,
            notes: "".to_string(),
            deadline: Default::default(),
            time_spent: 0,
            being_modified: false,
            word_count: 0,
            para_count: 0,
            unique_words: Default::default(),
        }
    }
    fn set_note() {}
    fn set_deadline() {}
    fn calculate() {
        // calc word count, para count, char count, white space count
        // let path = Path::new(file_path);
        // let file = File::open(&path)?;
        // let reader = BufReader::new(file);
        // let mut total_size = 0;
        // let mut word_count = 0;
        // for line in reader.lines() {
        //     let line = line?;
        //     total_size += line.as_bytes().len();
        //     word_count += line.split_whitespace().count();
    }
    fn update_modified() {
        // if last modified is less than ?
        todo!()
    }
}
