pub fn has_extension(filename: &str, extension: &str) -> bool {
    filename
        .to_ascii_lowercase()
        .ends_with(&extension.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::has_extension;

    #[test]
    fn all_tests() {
        assert!(has_extension("test.txt", "txt"));
        assert!(has_extension("test.txt", ".txt"));
        assert!(!has_extension("test.txt", "other"));
        assert!(!has_extension("test.txt", ".other"));
    }
}
