use crate::platform::Os;
use std::fmt::Display;
use std::path::Path;

/// the unix name of an executable
#[derive(Clone, Debug, PartialEq)]
pub struct Unix(String);

impl Unix {
  /// provides the platform-specific version of this `UnixExecutableName`
  pub fn platform_path(self, os: Os) -> Platform {
    Platform(match os {
      Os::Linux | Os::MacOS => self.0,
      Os::Windows => format!("{self}.exe"),
    })
  }
}

impl Display for Unix {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}

impl From<&str> for Unix {
  fn from(value: &str) -> Self {
    Unix(value.to_string())
  }
}

impl From<String> for Unix {
  fn from(value: String) -> Self {
    Unix(value)
  }
}

/// The platform-specific filename of an executable.
/// On Windows: "unix-executable-name.exe"
pub struct Platform(String);

impl AsRef<Path> for Platform {
  fn as_ref(&self) -> &Path {
    Path::new(&self.0)
  }
}

impl Display for Platform {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}
