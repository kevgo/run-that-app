pub fn has_extension(filename: &str, extension: &str) -> bool {
    filename
        .to_ascii_lowercase()
        .ends_with(&extension.to_ascii_lowercase())
}
