use super::ExecutableNamePlatform;
use crate::platform::Os;
use std::fmt::Display;

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
