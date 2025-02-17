use super::add_paths;
use crate::prelude::*;
use std::path::Path;
use std::process::Command;

pub(crate) fn capture_output(executable: &Path, args: &[&str]) -> Result<String> {
  let mut cmd = Command::new(executable);
  cmd.args(args);
  #[allow(clippy::unwrap_used)] // there is always a parent here since this is a location inside the yard
  add_paths(&mut cmd, &[executable.parent().unwrap()]);
  let output = match cmd.output() {
    Ok(output) => output,
    Err(err) => {
      return Err(UserError::ExecutableCannotExecute {
        executable: executable.to_string_lossy().to_string(),
        err: err.to_string(),
      });
    }
  };
  let stdout = String::from_utf8_lossy(&output.stdout);
  let stderr = String::from_utf8_lossy(&output.stderr);
  let output = format!("{stdout}{stderr}");
  Ok(output)
}
