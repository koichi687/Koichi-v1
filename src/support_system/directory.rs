use super::{docx, img, pdf, pptx};
use std::fs;

pub fn scan_read(dir_path: &str) -> String {
    let mut output = String::new();

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                let file_name = path.file_name().unwrap_or_default().to_string_lossy();

                let features = match ext.to_lowercase().as_str() {
                    "docx" => docx::read(&path),
                    "pdf" => pdf::read(&path),
                    "pptx" => pptx::read(&path),
                    "png" | "jpg" | "jpeg" | "wedp" => img::read(&path),
                    _ => fs::read_to_string(&path).unwrap_or_default(),
                };

                if !features.trim().is_empty() {
                    output.push_str(&format!("\n\n--- [{}] ---\n{}", file_name, features));
                }
            }
        }
    }

    output
}
