use super::{add_paths, exit_status_to_code, render_call};
use crate::error::{Result, UserError};
use crate::executables::{Executable, ExecutableCall};
use std::process::{Command, ExitCode};

/// Runs the given executable with the given arguments.
/// Streams output to the user's terminal.
#[allow(clippy::unwrap_used)]
pub(crate) fn stream_output(executable: &Executable, args: &[String], apps_to_include: &[ExecutableCall]) -> Result<ExitCode> {
  let mut cmd = Command::new(executable);
  cmd.args(args);
  let mut paths_to_include = vec![executable.as_path().parent().unwrap()];
  for app_to_include in apps_to_include {
    paths_to_include.push(app_to_include.executable.as_path().parent().unwrap());
  }
  add_paths(&mut cmd, &paths_to_include);
  let exit_status = cmd.status().map_err(|err| UserError::CannotExecuteBinary {
    call: render_call(executable, args),
    reason: err.to_string(),
  })?;
  Ok(exit_status_to_code(exit_status))
}
