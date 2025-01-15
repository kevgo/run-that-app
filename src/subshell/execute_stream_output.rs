use super::{exit_status_to_code, format_call};
use crate::installation::run_other_executable::CallSignature;
use crate::prelude::*;
use std::env;
use std::ffi::OsString;
use std::process::{Command, ExitCode};

/// Runs the given executable with the given arguments.
/// Streams output to the user's terminal.
#[allow(clippy::unwrap_used)]
pub fn execute_stream_output(call_signature: &CallSignature, args: &[String]) -> Result<ExitCode> {
  let mut cmd = Command::new(&call_signature.executable);
  cmd.args(&call_signature.args);
  cmd.args(args);
  cmd.envs(env::vars_os());
  let parent = call_signature.executable.0.parent().unwrap(); // there is always a parent here since this is a location inside the yard
  let new_path = if let Some(mut path) = env::var_os("PATH") {
    path.push(":");
    path.push(parent.as_os_str());
    path
  } else {
    OsString::from(parent)
  };
  cmd.env("PATH", new_path);
  let exit_status = cmd.status().map_err(|err| UserError::CannotExecuteBinary {
    call: format_call(&call_signature.executable, args),
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
      use crate::installation::run_other_executable::CallSignature;
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
      let call_signature = CallSignature {
        executable: Executable(executable_path),
        args: vec![],
      };
      let have = execute_stream_output(&call_signature, &[]).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(unix_exit_status(0))"));
    }

    #[test]
    #[cfg(unix)]
    fn unix_error() {
      use crate::filesystem::make_file_executable;
      use crate::installation::run_other_executable::CallSignature;
      use crate::subshell::{execute_stream_output, Executable};
      use big_s::S;
      use std::fs;
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable");
      fs::write(&executable_path, b"#!/bin/sh\nexit 3").unwrap();
      make_file_executable(&executable_path).unwrap();
      let call_signature = CallSignature {
        executable: Executable(executable_path),
        args: vec![],
      };
      let have = execute_stream_output(&call_signature, &[]).unwrap();
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
