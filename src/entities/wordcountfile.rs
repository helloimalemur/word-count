use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct WordCountFile {
    pub path: String,
    pub notes: String,
    pub deadline: DateTime<Utc>,
    pub time_spent: i128,
    pub being_modified: bool,
    pub word_count: i128,
    pub para_count: i32,
    pub unique_words: HashMap<String, i32>,
    pub full_file_contents: String,
}

impl WordCountFile {
    pub fn new(path: String) -> WordCountFile {
        WordCountFile {
            path,
            notes: "".to_string(),
            deadline: Default::default(),
            time_spent: 0,
            being_modified: false,
            word_count: 0,
            para_count: 0,
            unique_words: Default::default(),
            full_file_contents: "".to_string(),
        }
    }
}
