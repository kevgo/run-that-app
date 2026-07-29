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
  pub args: Option<Vec<String>>,

  /// the PATH environment variable to use when running this command
  pub env_path: Option<OsString>,
}

impl From<&CommandInfo> for Command {
  fn from(value: &CommandInfo) -> Self {
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

impl Display for CommandInfo {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(self.executable.to_string_lossy().as_ref())?;
    if let Some(args) = &self.args {
      for arg in args {
        f.write_char(' ')?;
        f.write_str(arg)?;
      }
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  mod display {
    use crate::CommandInfo;
    use big_s::S;

    #[test]
    fn no_args() {
      let cmd_info = CommandInfo {
        executable: "executable".into(),
        args: None,
        env_path: None,
      };
      let have = cmd_info.to_string();
      let want = S("executable");
      assert_eq!(have, want);
    }

    #[test]
    fn with_args() {
      let cmd_info = CommandInfo {
        executable: "executable".into(),
        args: Some(vec![S("arg1"), S("arg2")]),
        env_path: None,
      };
      let have = cmd_info.to_string();
      let want = S("executable arg1 arg2");
      assert_eq!(have, want);
    }
  }
}
