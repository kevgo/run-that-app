use super::executable_path::add_paths;
use super::{exit_status_to_code, ExecutableArgs, ExecutablePath};
use crate::prelude::*;
use std::fmt::{Display, Write};
use std::path::Path;
use std::process::{Command, ExitCode};

/// information to call an `App`s executable, as it is defined by the user
#[derive(Clone)]
pub(crate) struct ExecutableCallDefinition {
  pub(crate) executable_path: ExecutablePath,
  pub(crate) args: ExecutableArgs,
}

impl ExecutableCallDefinition {
  pub(crate) fn into_executable_call(self, app_folder: &Path) -> Option<ExecutableCall> {
    match self.args {
      ExecutableArgs::None => Some(ExecutableCall {
        executable_path: self.executable_path,
        args: vec![],
      }),
      ExecutableArgs::OneOfTheseInAppFolder { options } => {
        for option in options {
          let full_path = app_folder.join(option);
          if full_path.exists() {
            return Some(ExecutableCall {
              executable_path: self.executable_path,
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
    f.write_str(&self.executable_path.as_str())?;
    f.write_str(&self.args.to_string())?;
    Ok(())
  }
}

/// information to call an app with file paths adjusted
pub(crate) struct ExecutableCall {
  pub(crate) executable_path: ExecutablePath,
  pub(crate) args: Vec<String>,
}

impl ExecutableCall {
  /// provides a printable version of this `ExecutableCall` when called with additional arguments
  pub(crate) fn format_with_extra_args(&self, args: &[String]) -> String {
    let mut result = self.to_string();
    for arg in args {
      result.push(' ');
      result.push_str(arg);
    }
    result
  }

  /// Runs the given executable with the given arguments.
  /// Streams output to the user's terminal.
  #[allow(clippy::unwrap_used)]
  pub(crate) fn stream_output(&self, args: &[String], apps_to_include: &[ExecutableCall]) -> Result<ExitCode> {
    let mut cmd = Command::new(&self.executable_path);
    cmd.args(&self.args);
    cmd.args(args);
    let mut paths_to_include = vec![self.executable_path.as_path().parent().unwrap()];
    for app_to_include in apps_to_include {
      paths_to_include.push(app_to_include.executable_path.as_path().parent().unwrap());
    }
    add_paths(&mut cmd, &paths_to_include);
    let exit_status = cmd.status().map_err(|err| UserError::CannotExecuteBinary {
      call: self.format_with_extra_args(args),
      reason: err.to_string(),
    })?;
    Ok(exit_status_to_code(exit_status))
  }
}

impl Display for ExecutableCall {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(&self.executable_path.as_str())?;
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
  use crate::run::ExecutablePath;
  use big_s::S;
  use std::path::Path;

  mod stream_output {
    use crate::run::{ExecutableCall, ExecutablePath};
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
      let executable_call = ExecutableCall {
        executable_path: ExecutablePath::from(executable_path),
        args: vec![],
      };
      let have = executable_call.stream_output(&[], &[]).unwrap();
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
      let executable_call = ExecutableCall {
        executable_path: ExecutablePath::from(executable_path),
        args: vec![],
      };
      let have = executable_call.stream_output(&[], &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(unix_exit_status(3))"));
    }

    #[test]
    #[cfg(windows)]
    fn windows_success() {
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable.cmd");
      fs::write(&executable_path, b"echo hello").unwrap();
      let executable_call = ExecutableCall {
        executable_path: ExecutablePath::from(executable_path),
        args: vec![],
      };
      let have = stream_output(&executable_call, &[], &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(ExitCode(0))"));
    }

    #[test]
    #[cfg(windows)]
    fn windows_error() {
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable.cmd");
      fs::write(&executable_path, b"EXIT 3").unwrap();
      let executable_call = ExecutableCall {
        executable_path: ExecutablePath::from(executable_path),
        args: vec![],
      };
      let have = stream_output(&executable_call, &[], &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(ExitCode(3))"));
    }
  }

  #[test]
  fn format_with_extra_args() {
    let call = ExecutableCall {
      executable_path: ExecutablePath::from(Path::new("executable")),
      args: vec![S("arg1"), S("arg2")],
    };
    let have = call.format_with_extra_args(&[S("arg3")]);
    let want = S("executable arg1 arg2 arg3");
    assert_eq!(have, want);
  }

  #[test]
  fn to_string() {
    let call = ExecutableCall {
      executable_path: ExecutablePath::from(Path::new("executable")),
      args: vec![S("arg1"), S("arg2")],
    };
    let have = call.to_string();
    let want = S("executable arg1 arg2");
    assert_eq!(have, want);
  }
}
