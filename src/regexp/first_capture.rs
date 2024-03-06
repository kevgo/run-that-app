use regex::Regex;

/// provides the first capture of the given regex in the given text
pub fn first_capture<'a>(text: &'a str, regex: &str) -> Option<&'a str> {
    Some(Regex::new(regex).unwrap().captures(text)?.get(1)?.as_str())
}

#[cfg(test)]
mod tests {
    use super::first_capture;

    #[test]
    fn multiple_matches() {
        let text = "You can run 1.1 or 1.2 or 1.3";
        let have = first_capture(text, r"(\d+\.\d+)");
        let want = Some("1.1");
        assert_eq!(have, want);
    }

    #[test]
    fn no_match() {
        let text = "Foo bar";
        let have = first_capture(text, r"(\d+\.\d+)");
        let want = None;
        assert_eq!(have, want);
    }
}
