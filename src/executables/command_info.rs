use std::env;
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;

/// a command to execute, in a form that allows getting data
#[derive(Debug)]
pub struct CommandInfo {
  /// the executable to run
  pub executable: PathBuf,

  /// the arguments to pass to the executable
  pub args: Option<Vec<String>>,

  /// the PATH environment variable to use when running this command
  pub env_path: Option<OsString>,
}

impl From<CommandInfo> for Command {
  fn from(value: CommandInfo) -> Self {
    let CommandInfo { executable, args, env_path } = value;
    let mut cmd = Command::new(executable);
    if let Some(args) = args {
      cmd.args(args);
    }
    if let Some(env_path) = env_path {
      cmd.envs(env::vars_os());
      cmd.env("PATH", env_path);
    }
    cmd
  }
}
