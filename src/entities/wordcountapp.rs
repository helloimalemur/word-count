use std::sync::{Arc, Mutex, MutexGuard};
use crate::entities::wordcountfile::WordCountFile;

pub struct WordCountApp {
    pub files: Arc<Mutex<Vec<WordCountFile>>>

}

impl WordCountApp {
    pub fn new() -> WordCountApp {
        WordCountApp { files: Arc::new(Mutex::new(vec![])) }
    }
    fn add_file() {}
    fn remove_file() {}
    fn get_files(&self) -> Vec<WordCountFile> {
        self.files.lock().unwrap().clone()
    }
}
