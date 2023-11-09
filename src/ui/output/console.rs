use super::Output;
use std::io::{self, Write};

pub struct ConsoleOutput {
    pub category: Option<String>,
}

impl Output for ConsoleOutput {
    /// conditional logging of internal details
    fn log(&self, category: &str, text: &str) {
        if self.should_log(category) {
            self.println(&format!("{category}: {text}"));
        }
    }

    fn print(&self, text: &str) {
        print!("{}", text);
    }

    fn println(&self, text: &str) {
        println!("{}", text);
    }

    fn flush(&self) {
        let _ = io::stdout().flush();
    }
}

impl ConsoleOutput {
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
        use crate::ui::output::ConsoleOutput;
        use big_s::S;

        #[test]
        fn no_category() {
            let output = ConsoleOutput { category: None };
            assert!(!output.should_log("foo"));
            assert!(!output.should_log("bar"));
            assert!(!output.should_log(""));
        }

        #[test]
        fn empty_category() {
            let output = ConsoleOutput {
                category: Some(S("")),
            };
            assert!(output.should_log("foo"));
            assert!(output.should_log("bar"));
            assert!(output.should_log(""));
        }

        #[test]
        fn top_level_category() {
            let output = ConsoleOutput {
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
            let output = ConsoleOutput {
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
