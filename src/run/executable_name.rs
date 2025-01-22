use crate::platform::Os;
use std::fmt::Display;
use std::path::Path;

/// the unix name of an executable
#[derive(Clone, Debug, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct ExecutableNameUnix(String);

impl ExecutableNameUnix {
  /// provides the platform-specific version of this `UnixExecutableName`
  pub fn platform_path(self, os: Os) -> ExecutableFileName {
    ExecutableFileName(match os {
      Os::Linux | Os::MacOS => self.0,
      Os::Windows => format!("{self}.exe"),
    })
  }
}

impl Display for ExecutableNameUnix {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}

impl From<&str> for ExecutableNameUnix {
  fn from(value: &str) -> Self {
    ExecutableNameUnix(value.to_string())
  }
}

impl From<String> for ExecutableNameUnix {
  fn from(value: String) -> Self {
    ExecutableNameUnix(value)
  }
}

/// The platform-specific filename of an executable.
/// On Windows: "unix-executable-name.exe"
pub struct ExecutableFileName(String);

impl AsRef<Path> for ExecutableFileName {
  fn as_ref(&self) -> &Path {
    Path::new(&self.0)
  }
}

impl Display for ExecutableFileName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0)
  }
}
