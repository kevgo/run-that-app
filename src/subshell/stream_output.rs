use super::{add_paths, exit_status_to_code};
use crate::prelude::*;
use crate::run::ExecutableCall;
use std::process::{Command, ExitCode};

/// Runs the given executable with the given arguments.
/// Streams output to the user's terminal.
#[allow(clippy::unwrap_used)]
pub(crate) fn stream_output(executable: &ExecutableCall, args: &[String], apps_to_include: &[ExecutableCall]) -> Result<ExitCode> {
  let mut cmd = Command::new(&executable.executable_path);
  cmd.args(&executable.args);
  cmd.args(args);
  let mut paths_to_include = vec![executable.executable_path.as_path().parent().unwrap()];
  for app_to_include in apps_to_include {
    paths_to_include.push(app_to_include.executable_path.as_path().parent().unwrap());
  }
  add_paths(&mut cmd, &paths_to_include);
  let exit_status = cmd.status().map_err(|err| UserError::CannotExecuteBinary {
    call: executable.format_with_extra_args(args),
    reason: err.to_string(),
  })?;
  Ok(exit_status_to_code(exit_status))
}
