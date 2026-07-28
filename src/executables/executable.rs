use crate::error::Result;
use crate::logging::{Event, Log};
use crate::subshell;
use crate::subshell::shell_script_call;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::process::Command;

/// the full path to an executable that RTA knows exists and that it can execute
#[derive(Clone, Debug, PartialEq)]
pub enum Executable {
  /// the executable is a binary file and can be run directly
  Binary(PathBuf),
  /// the executable is a shell script and needs to be run through the default system shell
  ShellScript(PathBuf),
}

impl AsRef<OsStr> for Executable {
  fn as_ref(&self) -> &OsStr {
    match self {
      Executable::Binary(path) | Executable::ShellScript(path) => path.as_os_str(),
    }
  }
}

impl Executable {
  pub fn as_str(&self) -> Cow<'_, str> {
    match self {
      Executable::Binary(path) | Executable::ShellScript(path) => path.to_string_lossy(),
    }
  }

  /// runs this executable with the given args and returns the output it produced
  pub fn run_output(&self, args: &[&str], log: Log) -> Result<String> {
    log(Event::AnalyzeExecutableBegin { cmd: &self.as_str(), args });
    match self {
      Executable::Binary(path) => subshell::capture_output(self, args),
      Executable::ShellScript(path) => todo!(),
    }
  }

  pub fn as_path(&self) -> &Path {
    match self {
      Executable::Binary(path) | Executable::ShellScript(path) => path,
    }
  }

  pub fn parent_path(&self) -> &Path {
    #[allow(clippy::unwrap_used)] // there is always a parent here since this is a location inside the yard
    self.as_path().parent().unwrap()
  }
}

impl Display for Executable {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Executable::Binary(path) | Executable::ShellScript(path) => f.write_str(&path.to_string_lossy()),
    }
  }
}

impl From<Executable> for PathBuf {
  fn from(val: Executable) -> Self {
    match val {
      Executable::Binary(path) | Executable::ShellScript(path) => path,
    }
  }
}

impl From<Executable> for Command {
  fn from(value: Executable) -> Self {
    match value {
      Executable::Binary(path) => Command::new(path),
      Executable::ShellScript(path) => shell_script_call(&path, &[]),
    }
  }
}
