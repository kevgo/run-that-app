pub struct Logger {
    pub category: Option<String>,
}

impl Logger {
    fn log(&self, category: &str, text: &str) {
        if self.should_print(category) {
            println!("{category}: {text}");
        }
    }

    fn should_print(&self, mask: &str) -> bool {
        if let Some(category) = &self.category {
            category.is_empty() || mask.starts_with(category)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {

    mod console_logger {
        use crate::Logger;
        use big_s::S;

        #[test]
        fn no_category() {
            let logger = Logger { category: None };
            assert!(!logger.should_print("foo"));
            assert!(!logger.should_print("bar"));
            assert!(!logger.should_print(""));
        }

        #[test]
        fn empty_category() {
            let logger = Logger {
                category: Some(S("")),
            };
            assert!(logger.should_print("foo"));
            assert!(logger.should_print("bar"));
            assert!(logger.should_print(""));
        }

        #[test]
        fn top_level_category() {
            let logger = Logger {
                category: Some(S("detect")),
            };
            assert!(logger.should_print("detect"));
            assert!(logger.should_print("detect/os"));
            assert!(logger.should_print("detect/cpu"));
            assert!(!logger.should_print("other"));
            assert!(!logger.should_print("other/category"));
        }

        #[test]
        fn sub_category() {
            let logger = Logger {
                category: Some(S("detect/os")),
            };
            assert!(!logger.should_print("detect"));
            assert!(logger.should_print("detect/os"));
            assert!(!logger.should_print("detect/cpu"));
            assert!(!logger.should_print("other"));
            assert!(!logger.should_print("other/category"));
        }
    }
}
