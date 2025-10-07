use super::Executable;
use crate::error::{Result, UserError};
use crate::installation::BinFolder;
use std::fmt::{Display, Write};
use std::path::Path;

/// information to call an `App`s executable, as it is defined by the user
#[derive(Clone)]
pub(crate) struct ExecutableCallDefinition {
  pub(crate) executable: Executable,
  pub(crate) args: ExecutableArgs,
}

impl ExecutableCallDefinition {
  pub(crate) fn into_executable_call(self, app_folder: &Path) -> Option<ExecutableCall> {
    match self.args {
      ExecutableArgs::None => Some(ExecutableCall {
        executable: self.executable,
        args: vec![],
      }),
      ExecutableArgs::OneOfTheseInAppFolder { options } => {
        for option in options {
          let full_path = app_folder.join(option);
          if full_path.exists() {
            return Some(ExecutableCall {
              executable: self.executable,
              args: vec![full_path.to_string_lossy().to_string()],
            });
          }
        }
        None
      }
    }
  }
}

impl Display for ExecutableCallDefinition {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.executable.as_str())?;
    self.args.fmt(f)?;
    Ok(())
  }
}

/// Arguments that are required to execute an application itself - these are not arguments provided by the user.
/// Example: running npm happens as "node npm.js", "npm.js" is the executable arg.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum ExecutableArgs {
  /// the executable is called without any additional arguments
  None,
  /// uses the first of the given options that exists inside the folder that application is installed in
  OneOfTheseInAppFolder { options: Vec<&'static str> },
}

impl ExecutableArgs {
  /// provides the argument to use, adjusted to a callable format
  pub(crate) fn locate(&self, app_folder: &Path, bin_folder: &BinFolder) -> Result<Vec<String>> {
    match self {
      ExecutableArgs::None => Ok(vec![]),
      ExecutableArgs::OneOfTheseInAppFolder { options } => {
        for bin_folder_path in &bin_folder.possible_paths(app_folder) {
          for option in options {
            let absolute_path = bin_folder_path.join(option);
            if absolute_path.exists() {
              return Ok(vec![absolute_path.to_string_lossy().to_string()]);
            }
          }
        }
        Err(UserError::CannotFindExecutable)
      }
    }
  }
}

impl Display for ExecutableArgs {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ExecutableArgs::None => f.write_str("no args"),
      ExecutableArgs::OneOfTheseInAppFolder { options } => {
        f.write_str("one of these filesystem entries:")?;
        for option in options {
          f.write_char(' ')?;
          f.write_str(option)?;
        }
        Ok(())
      }
    }
  }
}

/// information to call an app with file paths adjusted
pub(crate) struct ExecutableCall {
  pub(crate) executable: Executable,
  pub(crate) args: Vec<String>,
}

impl ExecutableCall {
  /// provides the data to call this `ExecutableCall` with the given arguments
  pub(crate) fn with_args(self, mut args: Vec<String>) -> (Executable, Vec<String>) {
    let mut result_args = self.args;
    result_args.append(&mut args);
    (self.executable, result_args)
  }
}

impl Display for ExecutableCall {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.executable.as_str())?;
    for arg in &self.args {
      f.write_char(' ')?;
      f.write_str(arg)?;
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::ExecutableCall;
  use crate::executables::Executable;
  use big_s::S;
  use std::path::Path;

  mod stream_output {
    use crate::executables::Executable;
    use crate::subshell;
    use big_s::S;
    use std::fs;

    #[test]
    #[cfg(unix)]
    fn unix_success() {
      use std::io::Write;
      use std::os::unix::fs::PermissionsExt;
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable");
      let mut file = fs::File::create(&executable_path).unwrap();
      file.write_all(b"#!/bin/sh\necho hello").unwrap();
      file.set_permissions(fs::Permissions::from_mode(0o744)).unwrap();
      file.flush().unwrap();
      drop(file);
      // NOTE: if the test is flaky, wait 10 ms here.
      let executable = Executable::from(executable_path);
      let have = subshell::stream_output(&executable, &[], &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(unix_exit_status(0))"));
    }

    #[test]
    #[cfg(unix)]
    fn unix_error() {
      use crate::filesystem::make_file_executable;
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable");
      fs::write(&executable_path, b"#!/bin/sh\nexit 3").unwrap();
      make_file_executable(&executable_path).unwrap();
      let executable = Executable::from(executable_path);
      let have = subshell::stream_output(&executable, &[], &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(unix_exit_status(3))"));
    }

    #[test]
    #[cfg(windows)]
    fn windows_success() {
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable.cmd");
      fs::write(&executable_path, b"echo hello").unwrap();
      let executable = Executable::from(executable_path);
      let have = subshell::stream_output(&executable, &[], &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(ExitCode(0))"));
    }

    #[test]
    #[cfg(windows)]
    fn windows_error() {
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable.cmd");
      fs::write(&executable_path, b"EXIT 3").unwrap();
      let executable = Executable::from(executable_path);
      let have = subshell::stream_output(&executable, &[], &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(ExitCode(3))"));
    }
  }

  #[test]
  fn to_string() {
    let call = ExecutableCall {
      executable: Executable::from(Path::new("executable")),
      args: vec![S("arg1"), S("arg2")],
    };
    let have = call.to_string();
    let want = S("executable arg1 arg2");
    assert_eq!(have, want);
  }
}
