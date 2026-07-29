use std::env;
use std::ffi::OsString;
use std::fmt::{Display, Write};
use std::path::PathBuf;
use std::process::Command;

/// a command to execute, in a form that allows getting data
#[derive(Clone, Debug, PartialEq)]
pub struct CommandInfo {
  /// the executable to run
  pub executable: PathBuf,

  /// the arguments to pass to the executable
  pub args: Vec<String>,

  /// the PATH environment variable to use when running this command
  pub env_path: OsString,
}

impl From<&CommandInfo> for Command {
  fn from(value: &CommandInfo) -> Self {
    let CommandInfo { executable, args, env_path } = value;
    let mut cmd = Command::new(executable);
    cmd.args(args);
    cmd.envs(env::vars_os());
    cmd.env("PATH", env_path);
    cmd
  }
}

impl Display for CommandInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.executable.to_string_lossy().as_ref())?;
    for arg in &self.args {
      f.write_char(' ')?;
      f.write_str(arg)?;
    }
    Ok(())
  }
}
