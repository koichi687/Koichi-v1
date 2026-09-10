use crate::request::MessageSaving;
use std::fs;

// cara kerja -> buat file otomatis melalui kode di bawah
const MEMORY_DIR: &str = "storage/memory";
const DOCS_DIR: &str = "storage/docs";
const HISTORY_FILE: &str = "storage/training.json";

pub fn init() {
    let _ = fs::create_dir_all(MEMORY_DIR);
    let _ = fs::create_dir_all(DOCS_DIR);
}

//lalu di simpen
pub fn save_history(history: &[MessageSaving]) {
    if let Ok(json_str) = serde_json::to_string_pretty(history) {
        let _ = fs::write(HISTORY_FILE, json_str);
    }
}

pub fn load() -> Vec<MessageSaving> {
    match fs::read_to_string(HISTORY_FILE) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}
