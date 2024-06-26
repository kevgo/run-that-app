use std::fmt::Display;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct AppName(String);

impl AppName {
  pub fn as_str(&self) -> &str {
    &self.0
  }

  pub fn new(name: String) -> AppName {
    AppName(name)
  }
}

impl From<&str> for AppName {
  fn from(value: &str) -> Self {
    assert!(!value.is_empty(), "empty app name");
    assert!(value.to_lowercase() == value, "app name is not all lowercase");
    AppName::new(value.to_string())
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

impl PartialEq<&AppName> for AppName {
  fn eq(&self, other: &&AppName) -> bool {
    self == *other
  }
}

impl AsRef<Path> for AppName {
  fn as_ref(&self) -> &Path {
    Path::new(&self.0)
  }
}
