pub fn strip_filepath<'a>(filepath: &'a str, prefix: &str) -> &'a str {
    if !filepath.starts_with(prefix) {
        println!("FILEPATH {filepath} DOES NOT HAVE PREFIX {prefix}");
        return filepath;
    }
    &filepath[prefix.len()..]
}
