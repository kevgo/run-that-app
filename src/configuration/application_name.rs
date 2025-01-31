use std::fmt::Display;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub(crate) struct ApplicationName<'a>(&'a str);

impl<'a> ApplicationName<'a> {
  pub(crate) fn as_str(&self) -> &str {
    &self.0
  }

  pub(crate) fn new(name: &'a str) -> ApplicationName<'a> {
    ApplicationName(name)
  }
}

impl<'a> From<&'static str> for ApplicationName<'a> {
  fn from(value: &'static str) -> Self {
    ApplicationName(value)
  }
}

impl<'a> Display for ApplicationName<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}

impl<'a> PartialEq<&str> for ApplicationName<'a> {
  fn eq(&self, other: &&str) -> bool {
    self.0 == *other
  }
}

impl<'a> PartialEq<&ApplicationName> for ApplicationName<'a> {
  fn eq(&self, other: &ApplicationName) -> bool {
    self == other
  }
}

impl<'a> AsRef<Path> for ApplicationName<'a> {
  fn as_ref(&self) -> &Path {
    Path::new(&self.0)
  }
}
