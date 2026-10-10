use super::exit_status_to_code;
use crate::error::{Result, UserError};
use crate::executables::cmd_to_string;
use std::path::Path;
use std::process::{Command, ExitCode};

/// Runs the given command.
/// Streams output to the user's terminal.
pub fn stream_output(cmd: &mut Command, cwd: Option<&Path>) -> Result<ExitCode> {
  if let Some(dir) = cwd {
    cmd.current_dir(dir);
  }
  let exit_status = cmd.status().map_err(|err| UserError::CannotExecuteBinary {
    call: cmd_to_string(cmd),
    reason: err.to_string(),
  })?;
  Ok(exit_status_to_code(exit_status))
}
