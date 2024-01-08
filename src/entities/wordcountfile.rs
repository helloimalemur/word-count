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
    pub(crate) full_file_contents: String,
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
            full_file_contents: "".to_string(),
        }
    }
}
