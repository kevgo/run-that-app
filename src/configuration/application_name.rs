use std::fmt::Display;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub(crate) struct ApplicationName(&'static str);

impl ApplicationName {
  pub(crate) fn as_str(&self) -> &str {
    self.0
  }
}

impl From<&'static str> for ApplicationName {
  fn from(value: &'static str) -> Self {
    ApplicationName(value)
  }
}

impl Display for ApplicationName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.0)
  }
}

impl PartialEq<&str> for ApplicationName {
  fn eq(&self, other: &&str) -> bool {
    self.0 == *other
  }
}

impl AsRef<Path> for ApplicationName {
  fn as_ref(&self) -> &Path {
    Path::new(&self.0)
  }
}

impl AsRef<str> for ApplicationName {
  fn as_ref(&self) -> &str {
    self.0
  }
}
