use std::path::Path;
use std::process::Command;

#[cfg(not(windows))]
pub fn shell_script_call(shell_script: &Path, app_args: &[String]) -> Command {
  let mut args = Vec::with_capacity(app_args.len() + 1);
  args.push(shell_script.to_string_lossy().to_string());
  args.extend(app_args.iter().cloned());
  let mut command = Command::new("sh");
  command.arg("-c");
  command.args(args);
  command
}

#[cfg(windows)]
pub fn shell_script_call(shell_script: &Path, app_args: &[String]) -> Command {
  let mut args = Vec::with_capacity(app_args.len() + 1);
  args.push(shell_script.to_string_lossy().to_string());
  args.extend(app_args.iter().cloned());
  let mut command = Command::new("cmd");
  command.arg("/C");
  command.args(args);
  command
}
