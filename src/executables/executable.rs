use crate::subshell::shell_script_call;
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::process::Command;

/// the full path to an executable that RTA knows exists and that it can execute
#[derive(Clone, Debug, PartialEq)]
pub enum Executable {
  /// the executable is a binary file and can run directly
  Binary(PathBuf),
  /// the executable is a shell script and needs to run through the default system shell
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
  pub fn as_path(&self) -> &Path {
    match self {
      Executable::Binary(path) | Executable::ShellScript(path) => path,
    }
  }

  pub fn parent_path(&self) -> &Path {
    #[allow(clippy::unwrap_used)] // there is always a parent here since this is a location inside the yard
    self.as_path().parent().unwrap()
  }

  /// Builds a command that runs this executable with `app_args`.
  ///
  /// For a shell script, `app_args` are arguments of that script.
  pub fn into_command(self, app_args: &[String]) -> Command {
    match self {
      Executable::Binary(path) => {
        let mut cmd = Command::new(path);
        cmd.args(app_args);
        cmd
      }
      Executable::ShellScript(path) => shell_script_call(&path, app_args),
    }
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
