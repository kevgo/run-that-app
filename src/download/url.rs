use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Url(String);

impl AsRef<str> for Url {
  fn as_ref(&self) -> &str {
    &self.0
  }
}

impl From<Url> for String {
  fn from(val: Url) -> Self {
    val.0
  }
}

impl Display for Url {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}

impl From<String> for Url {
  fn from(value: String) -> Self {
    Url(value)
  }
}

impl From<&str> for Url {
  fn from(value: &str) -> Self {
    Url(value.to_string())
  }
}
