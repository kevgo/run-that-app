use crate::executables::ExecutableCall;
use big_s::S;
use std::path::Path;

#[cfg(not(windows))]
pub fn executable_call_for_shell_script(shell_script: &Path) -> ExecutableCall {
  ExecutableCall {
    executable: "sh".into(),
    args: vec![S("-c"), shell_script.to_string_lossy().to_string()],
  }
}

#[cfg(windows)]
pub fn executable_call_for_shell_script(shell_script: &Path) -> ExecutableCall {
  ExecutableCall {
    executable: "cmd".into(),
    args: vec![S("/C"), shell_script.to_string_lossy().to_string()],
  }
}
