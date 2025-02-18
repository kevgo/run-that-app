use super::add_paths;
use crate::prelude::*;
use crate::run::Executable;
use std::process::Command;

/// executes the given executable with the given args, returns the captured output (STDOUT and STDERR)
pub(crate) fn capture_output(executable: &Executable, args: &[&str]) -> Result<String> {
  let mut cmd = Command::new(executable);
  cmd.args(args);
  #[allow(clippy::unwrap_used)] // there is always a parent here since this is a location inside the yard
  add_paths(&mut cmd, &[executable.as_path().parent().unwrap()]);
  let output = match cmd.output() {
    Ok(output) => output,
    Err(err) => {
      return Err(UserError::ExecutableCannotExecute {
        executable: executable.to_string(),
        err: err.to_string(),
      });
    }
  };
  let stdout = String::from_utf8_lossy(&output.stdout);
  let stderr = String::from_utf8_lossy(&output.stderr);
  let output = format!("{stdout}{stderr}");
  Ok(output)
}
