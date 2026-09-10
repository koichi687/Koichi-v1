use std::path::Path;

pub fn read(path: &Path) -> String {
    let file = path.file_name().unwrap_or_default().to_string_lossy();
    format!("{}", file)
}
