use crate::platform::Os;
use std::fmt::Display;
use std::path::Path;

/// the unix name of an executable
#[derive(Clone, Debug, PartialEq)]
pub struct UnixExecutableName(String);

impl UnixExecutableName {
  /// provides the platform-specific version of this `UnixExecutableName`
  pub fn platform_path(self, os: Os) -> ExecutableFilename {
    ExecutableFilename(match os {
      Os::Linux | Os::MacOS => self.0,
      Os::Windows => format!("{self}.exe"),
    })
  }
}

impl Display for UnixExecutableName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}

impl From<&str> for UnixExecutableName {
  fn from(value: &str) -> Self {
    UnixExecutableName(value.to_string())
  }
}

impl From<String> for UnixExecutableName {
  fn from(value: String) -> Self {
    UnixExecutableName(value)
  }
}

/// The platform-specific filename of an executable.
/// On Windows: "unix-executable-name.exe"
pub struct ExecutableFilename(String);

impl AsRef<Path> for ExecutableFilename {
  fn as_ref(&self) -> &Path {
    Path::new(&self.0)
  }
}

impl Display for ExecutableFilename {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}
