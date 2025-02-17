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
pub(crate) struct ExecutablePath(PathBuf);

impl AsRef<OsStr> for ExecutablePath {
  fn as_ref(&self) -> &OsStr {
    self.0.as_os_str()
  }
}

impl ExecutablePath {
  pub(crate) fn as_str(&self) -> Cow<'_, str> {
    self.0.to_string_lossy()
  }

  /// runs this executable with the given args and returns the output it produced
  // TODO: use ExecutableCall internally?
  pub(crate) fn run_output(&self, args: &[&str], log: Log) -> Result<String> {
    let mut cmd = Command::new(self);
    cmd.args(args);
    #[allow(clippy::unwrap_used)] // there is always a parent here since this is a location inside the yard
    add_paths(&mut cmd, &[self.0.parent().unwrap()]);
    log(Event::AnalyzeExecutableBegin { cmd: &self.as_str(), args });
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

  pub(crate) fn inner(self) -> PathBuf {
    self.0
  }

  pub(crate) fn as_path(&self) -> &Path {
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

/// adds the given dirs to the PATH env variable of the given cmd
pub(crate) fn add_paths(cmd: &mut Command, dirs: &[&Path]) {
  cmd.envs(env::vars_os());
  let new_path = if let Some(mut path) = env::var_os("PATH") {
    // PATH env var is set to something here, could be empty string
    for dir in dirs {
      if !path.is_empty() {
        path.push(":");
      }
      path.push(dir.as_os_str());
    }
    path
  } else {
    // PATH env var is empty here
    let mut path = OsString::new();
    for dir in dirs {
      if !path.is_empty() {
        path.push(":");
      }
      path.push(dir);
    }
    path
  };
  cmd.env("PATH", new_path);
}
