use crate::platform::Os;
use std::fmt::Display;
use std::path::Path;

/// the unix name of an executable
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ExecutableNameUnix(String);

impl ExecutableNameUnix {
  /// provides the platform-specific version of this `UnixExecutableName`
  pub(crate) fn platform_path(self, os: Os) -> ExecutableNamePlatform {
    ExecutableNamePlatform::from(match os {
      Os::Linux | Os::MacOS => self.0,
      Os::Windows => format!("{self}.exe"),
    })
  }
}

impl AsRef<str> for ExecutableNameUnix {
  fn as_ref(&self) -> &str {
    &self.0
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
pub(crate) struct ExecutableNamePlatform(String);

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
