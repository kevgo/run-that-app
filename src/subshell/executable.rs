use crate::error::UserError;
use crate::logger::{Event, Log};
use crate::Result;
use std::borrow::Cow;
use std::ffi::OsStr;
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
        log(Event::AnalyzeExecutableQuery {
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
        let stdout = String::from_utf8(output.stdout).expect("command printed non unicode output");
        log(Event::AnalyzeExecutableOutput { output: &stdout });
        Ok(stdout)
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
        Ok(String::from_utf8(output.stdout).unwrap_or_default())
    }
}
