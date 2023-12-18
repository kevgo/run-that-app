pub fn strip_filepath<'a>(filepath: &'a str, prefix: &str) -> &'a str {
    &filepath[prefix.len()..]
}
