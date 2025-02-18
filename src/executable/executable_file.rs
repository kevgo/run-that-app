use crate::logging::{Event, Log};
use crate::prelude::*;
use crate::subshell;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};

/// the full path to an executable that RTA knows exists and that it can execute
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ExecutableFile(PathBuf);

impl AsRef<OsStr> for ExecutableFile {
  fn as_ref(&self) -> &OsStr {
    self.0.as_os_str()
  }
}

impl ExecutableFile {
  pub(crate) fn as_str(&self) -> Cow<'_, str> {
    self.0.to_string_lossy()
  }

  /// runs this executable with the given args and returns the output it produced
  pub(crate) fn run_output(&self, args: &[&str], log: Log) -> Result<String> {
    log(Event::AnalyzeExecutableBegin { cmd: &self.as_str(), args });
    subshell::capture_output(self, args)
  }

  pub(crate) fn inner(self) -> PathBuf {
    self.0
  }

  pub(crate) fn as_path(&self) -> &Path {
    &self.0
  }
}

impl Display for ExecutableFile {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0.to_string_lossy())
  }
}

impl From<PathBuf> for ExecutableFile {
  fn from(value: PathBuf) -> Self {
    ExecutableFile(value)
  }
}

impl From<&Path> for ExecutableFile {
  fn from(value: &Path) -> Self {
    ExecutableFile(value.to_path_buf())
  }
}
