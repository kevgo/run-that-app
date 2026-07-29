use super::Executable;
use std::fmt::{Display, Write};

/// information to call an app with file paths adjusted
#[derive(Debug, PartialEq)]
pub struct ExecutableCall {
  pub executable: Executable,
  pub args: Vec<String>,
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
    use crate::executables::{CommandInfo, Executable};
    use crate::subshell;
    use big_s::S;
    use std::fs;

    fn cmd_info_for(executable: &Executable) -> CommandInfo {
      CommandInfo {
        executable: executable.as_path().to_path_buf(),
        args: None,
        env_path: None,
      }
    }

    #[test]
    #[cfg(not(windows))]
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
      let have = subshell::stream_output(cmd_info_for(&executable), None).unwrap();
      // HACK: is there a better way to compare ExitCode?
      assert_eq!(format!("{have:?}"), S("ExitCode(unix_exit_status(0))"));
    }

    #[test]
    #[cfg(not(windows))]
    fn unix_error() {
      use crate::filesystem;
      let tempdir = tempfile::tempdir().unwrap();
      let executable_path = tempdir.path().join("executable");
      fs::write(&executable_path, b"#!/bin/sh\nexit 3").unwrap();
      filesystem::set_executable_bit(&executable_path);
      let executable = Executable::from(executable_path);
      let have = subshell::stream_output(cmd_info_for(&executable), None).unwrap();
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
      let have = subshell::stream_output(cmd_info_for(&executable), None).unwrap();
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
      let have = subshell::stream_output(cmd_info_for(&executable), None).unwrap();
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
