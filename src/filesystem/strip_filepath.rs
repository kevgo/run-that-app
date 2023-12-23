pub fn strip_filepath<'a>(filepath: &'a str, prefix: &str) -> &'a str {
    &filepath[prefix.len()..]
}

#[cfg(test)]
mod tests {
    use super::strip_filepath;

    #[test]
    fn normal() {
        let have = strip_filepath("node/README.md", "node/");
        let want = "README.md";
        assert_eq!(have, want);
    }

    #[test]
    fn exact_match() {
        let have = strip_filepath("node/", "node/");
        let want = "";
        assert_eq!(have, want);
    }
}
