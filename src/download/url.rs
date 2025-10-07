use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct URL(String);

impl AsRef<str> for URL {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl Into<String> for URL {
  fn into(self) -> String {
    self.0
  }
}

impl Display for URL {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}

impl From<String> for URL {
  fn from(value: String) -> Self {
    URL(value)
  }
}

impl From<&str> for URL {
  fn from(value: &str) -> Self {
    URL(value.to_string())
  }
}
