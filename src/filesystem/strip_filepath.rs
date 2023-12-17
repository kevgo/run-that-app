pub fn strip_filepath<'a>(filepath: &'a str, prefix: &str) -> &'a str {
    if !filepath.starts_with(prefix) {
        return filepath;
    }
    &filepath[prefix.len()..]
}
