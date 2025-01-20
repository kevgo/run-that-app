use std::fmt::Display;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct ApplicationName(String);

impl ApplicationName {
  pub fn as_str(&self) -> &str {
    &self.0
  }

  pub fn new(name: String) -> ApplicationName {
    ApplicationName(name)
  }
}

impl From<&str> for ApplicationName {
  fn from(value: &str) -> Self {
    assert!(!value.is_empty(), "empty app name");
    assert!(value.to_lowercase() == value, "app name is not all lowercase");
    ApplicationName::new(value.to_string())
  }
}

impl Display for ApplicationName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}

impl PartialEq<&str> for ApplicationName {
  fn eq(&self, other: &&str) -> bool {
    self.0 == *other
  }
}

impl PartialEq<&ApplicationName> for ApplicationName {
  fn eq(&self, other: &&ApplicationName) -> bool {
    self == *other
  }
}

impl AsRef<Path> for ApplicationName {
  fn as_ref(&self) -> &Path {
    Path::new(&self.0)
  }
}

impl ApplicationName {
  /// provides the underlying string value
  pub fn inner(self) -> String {
    self.0
  }
}
