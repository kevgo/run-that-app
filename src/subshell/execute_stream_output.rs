use super::executable::add_path;
use super::{exit_status_to_code, format_call};
use crate::prelude::*;
use crate::subshell::Executable;
use std::process::{Command, ExitCode};

/// Runs the given executable with the given arguments.
/// Streams output to the user's terminal.
#[allow(clippy::unwrap_used)]
pub fn execute_stream_output(executable: &Executable, args: &[String]) -> Result<ExitCode> {
  let mut cmd = Command::new(executable);
  cmd.args(args);
  add_path(&mut cmd, executable.0.parent().unwrap());
  let exit_status = cmd.status().map_err(|err| UserError::CannotExecuteBinary {
    call: format_call(executable, args),
    reason: err.to_string(),
  })?;
  Ok(exit_status_to_code(exit_status))
}

#[cfg(test)]
mod tests {
  mod execute {

    #[test]
    #[cfg(unix)]
    fn unix_success() {
      use crate::subshell::{execute_stream_output, Executable};
      use big_s::S;
      use std::io::Write;
      use std::os::unix::fs::PermissionsExt;
      use std::time::Duration;
      use std::{fs, thread};
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable");
      let mut file = fs::File::create(&executable_path).unwrap();
      file.write_all(b"#!/bin/sh\necho hello").unwrap();
      file.set_permissions(fs::Permissions::from_mode(0o744)).unwrap();
      drop(file);
      thread::sleep(Duration::from_millis(10)); // give the OS time to close the file to avoid a flaky test
      let have = execute_stream_output(&Executable(executable_path), &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(unix_exit_status(0))"));
    }

    #[test]
    #[cfg(unix)]
    fn unix_error() {
      use crate::filesystem::make_file_executable;
      use crate::subshell::{execute_stream_output, Executable};
      use big_s::S;
      use std::fs;
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable");
      fs::write(&executable_path, b"#!/bin/sh\nexit 3").unwrap();
      make_file_executable(&executable_path).unwrap();
      let executable = Executable(executable_path);
      let have = execute_stream_output(&executable, &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(unix_exit_status(3))"));
    }

    #[test]
    #[cfg(windows)]
    fn windows_success() {
      use crate::subshell::{execute_stream_output, Executable};
      use big_s::S;
      use std::fs;
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable.cmd");
      fs::write(&executable_path, b"echo hello").unwrap();
      let executable = Executable(executable_path);
      let have = execute_stream_output(&executable, &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(ExitCode(0))"));
    }

    #[test]
    #[cfg(windows)]
    fn windows_error() {
      use crate::subshell::{execute_stream_output, Executable};
      use big_s::S;
      use std::fs;
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable.cmd");
      fs::write(&executable_path, b"EXIT 3").unwrap();
      let executable = Executable(executable_path);
      let have = execute_stream_output(&executable, &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(ExitCode(3))"));
    }
  }
}
