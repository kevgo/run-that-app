use super::Output;
use std::io::{self, Write};

pub struct Console {
    pub category: Option<String>,
}

impl Output for Console {
    /// conditional logging of internal details
    fn log(&self, category: &str, text: &str) {
        if self.should_log(category) {
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

impl Console {
    pub fn should_log(&self, mask: &str) -> bool {
        if let Some(category) = &self.category {
            category.is_empty() || mask.starts_with(category)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {

    mod should_log {
        use crate::output::Console;
        use big_s::S;

        #[test]
        fn no_category() {
            let output = Console { category: None };
            assert!(!output.should_log("foo"));
            assert!(!output.should_log("bar"));
            assert!(!output.should_log(""));
        }

        #[test]
        fn empty_category() {
            let output = Console {
                category: Some(S("")),
            };
            assert!(output.should_log("foo"));
            assert!(output.should_log("bar"));
            assert!(output.should_log(""));
        }

        #[test]
        fn top_level_category() {
            let output = Console {
                category: Some(S("detect")),
            };
            assert!(output.should_log("detect"));
            assert!(output.should_log("detect/os"));
            assert!(output.should_log("detect/cpu"));
            assert!(!output.should_log("other"));
            assert!(!output.should_log("other/category"));
        }

        #[test]
        fn sub_category() {
            let output = Console {
                category: Some(S("detect/os")),
            };
            assert!(!output.should_log("detect"));
            assert!(output.should_log("detect/os"));
            assert!(!output.should_log("detect/cpu"));
            assert!(!output.should_log("other"));
            assert!(!output.should_log("other/category"));
        }
    }
}
