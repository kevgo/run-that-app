use crate::executables::ExecutableCall;
#[cfg(not(windows))]
use big_s::S;
#[cfg(not(windows))]
use std::path::Path;

#[cfg(not(windows))]
pub fn executable_call_for_shell_script(shell_script: &Path, app_args: &[String]) -> ExecutableCall {
  let mut args = vec![shell_script.to_string_lossy().to_string()];
  args.extend(app_args.iter().cloned());
  let arg = args.join(" ");
  ExecutableCall {
    executable: "sh".into(),
    args: vec![S("-c"), arg],
  }
}

#[cfg(windows)]
pub fn executable_call_for_shell_script(shell_script: &Path) -> ExecutableCall {
  use crate::executables::Executable;

  ExecutableCall {
    executable: Executable::new("cmd"),
    args: vec![S("/C"), shell_script.to_string_lossy().to_string()],
  }
}
