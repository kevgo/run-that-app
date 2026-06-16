use std::env;
use std::path::PathBuf;
use std::process::Command;

/// a command to execute, in a form that allows getting data
pub struct CommandInfo {
  /// the executable to run
  pub executable: PathBuf,

  /// the arguments to pass to the executable
  pub args: Vec<String>,

  /// the PATH environment variable to use when running this command
  pub env_path: Option<String>,
}

impl CommandInfo {
  pub fn to_command(&self) -> Command {
    let mut cmd = Command::new(&self.executable);
    cmd.args(&self.args);
    cmd.envs(env::vars_os());
    if let Some(env_path) = &self.env_path {
      cmd.env("PATH", env_path);
    }
    cmd
  }
}
