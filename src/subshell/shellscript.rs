use crate::executables::ExecutableCall;
use big_s::S;
use std::path::Path;

#[cfg(not(windows))]
pub fn shell_script_call(shell_script: &Path, app_args: &[String]) -> ExecutableCall {
  let mut args = vec![shell_script.to_string_lossy().to_string()];
  args.extend(app_args.iter().cloned());
  let arg = args.join(" ");
  ExecutableCall {
    executable: "sh".into(),
    args: vec![S("-c"), arg],
  }
}

#[cfg(windows)]
pub fn shell_script_call(shell_script: &Path) -> ExecutableCall {
  ExecutableCall {
    executable: "cmd".into(),
    args: vec![S("/C"), shell_script.to_string_lossy().to_string()],
  }
}
