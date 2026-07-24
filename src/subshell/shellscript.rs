#[cfg(not(windows))]
use std::path::Path;

use crate::error::Result;
use crate::executables::ExecutableCall;

#[cfg(not(windows))]
pub fn executable_call_for_shell_script(shell_script: &Path) -> Result<ExecutableCall> {
  Ok(ExecutableCall {
    executable: "sh".into(),
    args: vec![shell_script.to_string()],
  })
}

#[cfg(windows)]
pub fn executable_call_for_shell_script(shell_script: &str) -> ExecutableCall {
  ExecutableCall {
    executable: Executable::new("cmd"),
    args: vec!["/C", shell_script.to_string()],
  }
}
