use std::path::Path;

pub fn strip_filepath<'a>(filepath: &'a Path, prefix: &str) -> &'a Path {
    if !filepath.starts_with(prefix) {
        return filepath;
    }
    let text = filepath.to_string_lossy();
    let trimmed = &text[prefix.len()..];
    &Path::new(&trimmed)
}
