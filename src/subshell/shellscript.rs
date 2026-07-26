use crate::executables::ExecutableCall;
use big_s::S;
use std::path::Path;

#[cfg(not(windows))]
pub fn shell_script_call(shell_script: &Path, app_args: &[String]) -> ExecutableCall {
  let mut args = Vec::with_capacity(app_args.len() + 1);
  args.push(shell_script.to_string_lossy().to_string());
  args.extend(app_args.iter().cloned());
  ExecutableCall {
    executable: "sh".into(),
    args: vec![S("-c"), args.join(" ")],
  }
}

#[cfg(windows)]
pub fn shell_script_call(shell_script: &Path, app_args: &[String]) -> ExecutableCall {
  let mut args = Vec::with_capacity(app_args.len() + 1);
  args.push(shell_script.to_string_lossy().to_string());
  args.extend(app_args.iter().cloned());
  ExecutableCall {
    executable: "cmd".into(),
    args: vec![S("/C"), args.join(" ")],
  }
}
