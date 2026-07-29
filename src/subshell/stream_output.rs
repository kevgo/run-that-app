use super::exit_status_to_code;
use crate::error::{Result, UserError};
use crate::executables::CommandInfo;
use std::path::Path;
use std::process::{Command, ExitCode};

/// Runs the given command.
/// Streams output to the user's terminal.
pub fn stream_output(cmd_info: &CommandInfo, cwd: Option<&Path>) -> Result<ExitCode> {
  let mut cmd = Command::from(cmd_info);
  if let Some(dir) = cwd {
    cmd.current_dir(dir);
  }
  let exit_status = cmd.status().map_err(|err| UserError::CannotExecuteBinary {
    call: cmd_info.to_owned(),
    reason: err.to_string(),
  })?;
  Ok(exit_status_to_code(exit_status))
}
