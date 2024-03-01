use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct AppName(String);

impl From<&str> for AppName {
    fn from(value: &str) -> Self {
        AppName(value.to_string())
    }
}

impl Display for AppName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialEq<&str> for AppName {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}
