use super::Output;
use std::io::{self, Write};

pub struct StdErr {
    pub category: Option<String>,
}

impl Output for StdErr {
    fn is_active(&self, candidate: &str) -> bool {
        if let Some(category) = &self.category {
            category.is_empty() || candidate.starts_with(category)
        } else {
            false
        }
    }

    /// conditional logging of internal details
    fn log(&self, category: &str, text: &str) {
        if self.is_active(category) {
            self.println(&format!("{category}: {text}"));
        }
    }

    fn print(&self, text: &str) {
        eprint!("{text}");
        let _ = io::stderr().flush();
    }

    fn println(&self, text: &str) {
        eprintln!("{text}");
    }
}

#[cfg(test)]
mod tests {

    mod should_log {
        use crate::output::{Output, StdErr};
        use big_s::S;

        #[test]
        fn no_category() {
            let output = StdErr { category: None };
            assert!(!output.is_active("foo"));
            assert!(!output.is_active("bar"));
            assert!(!output.is_active(""));
        }

        #[test]
        fn empty_category() {
            let output = StdErr { category: Some(S("")) };
            assert!(output.is_active("foo"));
            assert!(output.is_active("bar"));
            assert!(output.is_active(""));
        }

        #[test]
        fn top_level_category() {
            let output = StdErr { category: Some(S("detect")) };
            assert!(output.is_active("detect"));
            assert!(output.is_active("detect/os"));
            assert!(output.is_active("detect/cpu"));
            assert!(!output.is_active("other"));
            assert!(!output.is_active("other/category"));
        }

        #[test]
        fn sub_category() {
            let output = StdErr {
                category: Some(S("detect/os")),
            };
            assert!(!output.is_active("detect"));
            assert!(output.is_active("detect/os"));
            assert!(!output.is_active("detect/cpu"));
            assert!(!output.is_active("other"));
            assert!(!output.is_active("other/category"));
        }
    }
}
