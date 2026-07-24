use crate::applications::ApplicationName;
use crate::platform::{Os, OsFamily};
use std::fmt::Display;
use std::path::Path;

/// the unix name of an executable
#[derive(Clone, Debug, PartialEq)]
pub struct ExecutableNameUnix(String);

impl ExecutableNameUnix {
  /// provides the platform-specific version of this `UnixExecutableName`
  pub fn platform_path(self, os: Os) -> ExecutableNamePlatform {
    match os {
      Os::Linux | Os::MacOS => ExecutableNamePlatform {
        name: self.0,
        os: OsFamily::Unix,
      },
      Os::Windows => ExecutableNamePlatform {
        name: format!("{self}.exe"),
        os: OsFamily::Windows,
      },
    }
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

impl From<ApplicationName> for ExecutableNameUnix {
  fn from(value: ApplicationName) -> Self {
    ExecutableNameUnix(value.to_string())
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
#[derive(Clone, Debug, PartialEq)]
pub struct ExecutableNamePlatform {
  /// the platform-specific name
  pub name: String,
  /// the operating system for which this name is valid
  pub os: OsFamily,
}

impl From<&str> for ExecutableNamePlatform {
  fn from(value: &str) -> Self {
    ExecutableNamePlatform(value.to_string())
  }
}

impl AsRef<Path> for ExecutableNamePlatform {
  fn as_ref(&self) -> &Path {
    Path::new(&self.name)
  }
}

impl Display for ExecutableNamePlatform {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.name)
  }
}
