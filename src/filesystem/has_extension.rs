pub fn has_extension(filename: &str, extension: &str) -> bool {
    let filename_len = filename.len();
    filename[filename_len - extension.len()..filename_len].eq_ignore_ascii_case(extension)
}

#[cfg(test)]
mod tests {
    use super::has_extension;

    #[test]
    fn all_tests() {
        assert!(has_extension("test.txt", "txt"));
        assert!(has_extension("test.txt", ".txt"));
        assert!(has_extension("test.tar.gz", ".tar.gz"));
        assert!(!has_extension("test.txt", "other"));
        assert!(!has_extension("test.txt", ".other"));
    }
}
