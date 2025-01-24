use std::fmt::Display;
use std::path::Path;

/// The platform-specific filename of an executable.
/// On Windows: "unix-executable-name.exe"
pub struct ExecutableNamePlatform(String);

impl From<String> for ExecutableNamePlatform {
  fn from(value: String) -> Self {
    ExecutableNamePlatform(value.to_string())
  }
}

impl AsRef<Path> for ExecutableNamePlatform {
  fn as_ref(&self) -> &Path {
    Path::new(&self.0)
  }
}

impl Display for ExecutableNamePlatform {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}
