use crate::executables::ExecutableCall;
#[cfg(not(windows))]
use big_s::S;
#[cfg(not(windows))]
use std::path::Path;

#[cfg(not(windows))]
pub fn executable_call_for_shell_script(shell_script: &Path) -> ExecutableCall {
  ExecutableCall {
    executable: "sh".into(),
    args: vec![S("-c"), shell_script.to_string_lossy().to_string()],
  }
}

#[cfg(windows)]
pub fn executable_call_for_shell_script(shell_script: &str) -> ExecutableCall {
  ExecutableCall {
    executable: Executable::new("cmd"),
    args: vec!["/C", shell_script.to_string()],
  }
}
