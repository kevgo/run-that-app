pub struct Output {
    pub category: Option<String>,
}

impl Output {
    pub fn log(&self, category: &str, text: &str) {
        if self.should_log(category) {
            println!("{category}: {text}");
        }
    }

    pub fn should_log(&self, mask: &str) -> bool {
        if let Some(category) = &self.category {
            category.is_empty() || mask.starts_with(category)
        } else {
            false
        }
    }

    pub fn print(&self, text: &str) {
        print!("{}", text);
    }

    pub fn println(&self, text: &str) {
        println!("{}", text);
    }
}

#[cfg(test)]
mod tests {

    mod should_log {
        use crate::Output;
        use big_s::S;

        #[test]
        fn no_category() {
            let output = Output { category: None };
            assert!(!output.should_log("foo"));
            assert!(!output.should_log("bar"));
            assert!(!output.should_log(""));
        }

        #[test]
        fn empty_category() {
            let output = Output {
                category: Some(S("")),
            };
            assert!(output.should_log("foo"));
            assert!(output.should_log("bar"));
            assert!(output.should_log(""));
        }

        #[test]
        fn top_level_category() {
            let output = Output {
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
            let output = Output {
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
