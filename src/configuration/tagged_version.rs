pub(crate) struct TaggedVersion(String);

impl TaggedVersion {
  pub(crate) fn as_str(&self) -> &str {
    &self.0
  }
}

impl From<String> for TaggedVersion {
  fn from(value: String) -> Self {
    TaggedVersion(value)
  }
}
