use crate::logging::{Event, Log};
use crate::prelude::*;
use std::borrow::Cow;
use std::env;
use std::ffi::{OsStr, OsString};
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::process::Command;

/// the full path to an executable that RTA can execute
#[derive(Clone, Debug, PartialEq)]
pub struct ExecutablePath(PathBuf);

impl AsRef<OsStr> for ExecutablePath {
  fn as_ref(&self) -> &OsStr {
    self.0.as_os_str()
  }
}

impl ExecutablePath {
  pub fn as_str(&self) -> Cow<'_, str> {
    self.0.to_string_lossy()
  }

  pub fn dir(&self) -> &Path {
    &self.0.parent().unwrap()
  }

  /// runs this executable with the given args and returns the output it produced
  // TODO: move this into a top-level function
  pub fn run_output(&self, arg: &str, log: Log) -> Result<String> {
    let mut cmd = Command::new(self);
    cmd.arg(arg);
    #[allow(clippy::unwrap_used)] // there is always a parent here since this is a location inside the yard
    add_path(&mut cmd, self.0.parent().unwrap());
    log(Event::AnalyzeExecutableBegin {
      cmd: &self.as_str(),
      args: &[arg],
    });
    let output = match cmd.output() {
      Ok(output) => output,
      Err(err) => {
        log(Event::AnalyzeExecutableError { err: err.to_string() });
        return Err(UserError::ExecutableCannotExecute {
          executable: self.clone(),
          err: err.to_string(),
        });
      }
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let output = format!("{stdout}{stderr}");
    Ok(output)
  }

  /// runs this executable with the given args and returns the output it produced
  // TODO: move this into a top-level function
  pub fn run_output_args(&self, args: &[&str], log: Log) -> Result<String> {
    let mut cmd = Command::new(self);
    cmd.args(args);
    #[allow(clippy::unwrap_used)] // there is always a parent here since this is a location inside the yard
    add_path(&mut cmd, self.0.parent().unwrap());
    let output = match cmd.output() {
      Ok(output) => output,
      Err(err) => {
        log(Event::AnalyzeExecutableError { err: err.to_string() });
        return Err(UserError::ExecutableCannotExecute {
          executable: self.clone(),
          err: err.to_string(),
        });
      }
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let output = format!("{stdout}{stderr}");
    Ok(output)
  }

  pub fn inner(self) -> PathBuf {
    self.0
  }

  pub fn as_path(&self) -> &Path {
    &self.0
  }
}

impl Display for ExecutablePath {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0.to_string_lossy())
  }
}

impl From<PathBuf> for ExecutablePath {
  fn from(value: PathBuf) -> Self {
    ExecutablePath(value)
  }
}

impl From<&Path> for ExecutablePath {
  fn from(value: &Path) -> Self {
    ExecutablePath(value.to_path_buf())
  }
}

/// adds the given dir to the PATH env variable of the given cmd
pub fn add_path(cmd: &mut Command, dir: &Path) {
  cmd.envs(env::vars_os());
  let new_path = if let Some(mut path) = env::var_os("PATH") {
    path.push(":");
    path.push(dir.as_os_str());
    path
  } else {
    OsString::from(dir)
  };
  cmd.env("PATH", new_path);
}
