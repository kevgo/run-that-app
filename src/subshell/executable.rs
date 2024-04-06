use crate::logger::{Event, Log};
use crate::prelude::*;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::Display;
use std::path::PathBuf;
use std::process::Command;

/// an application that is stored in the yard and can be executed
#[derive(Clone, Debug, PartialEq)]
pub struct Executable(pub PathBuf);

impl AsRef<OsStr> for Executable {
  fn as_ref(&self) -> &OsStr {
    self.0.as_os_str()
  }
}

impl Executable {
  pub fn as_str(&self) -> Cow<'_, str> {
    self.0.to_string_lossy()
  }

  /// runs this executable with the given args and returns the output it produced
  pub fn run_output(&self, arg: &str, log: Log) -> Result<String> {
    let mut cmd = Command::new(self);
    cmd.arg(arg);
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
    let stdout = String::from_utf8(output.stdout).expect("command printed non unicode stdout");
    let stderr = String::from_utf8(output.stderr).expect("command printed non unicode stderr");
    let output = format!("{stdout}{stderr}");
    Ok(output)
  }

  /// runs this executable with the given args and returns the output it produced
  pub fn run_output_args(&self, args: &[&str], log: Log) -> Result<String> {
    let mut cmd = Command::new(self);
    cmd.args(args);
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
    let stdout = String::from_utf8(output.stdout).expect("command printed non unicode stdout");
    let stderr = String::from_utf8(output.stderr).expect("command printed non unicode stderr");
    let output = format!("{stdout}{stderr}");
    Ok(output)
  }
}

impl Display for Executable {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.0.to_string_lossy())
  }
}
