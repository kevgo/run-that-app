//! This module implements various ways to execute work in subshells.

use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::process::{Command, ExitCode, ExitStatus};

mod capture_output;
mod detect_output;
mod stream_output;

use crate::executables::Executable;
pub(crate) use capture_output::capture_output;
pub(crate) use detect_output::detect_output;
pub(crate) use stream_output::stream_output;

/// adds the given dirs to the PATH env variable of the given cmd
pub(crate) fn add_paths(cmd: &mut Command, dirs: &[&Path]) {
  cmd.envs(env::vars_os());
  let new_path = if let Some(path) = env::var_os("PATH") {
    // PATH env var is set to something here, could be empty string
    prepend_paths(&path, dirs)
  } else {
    // PATH env var is empty here
    join_paths(dirs)
  };
  cmd.env("PATH", new_path);
}

fn prepend_paths(existing_paths: &OsString, dirs: &[&Path]) -> OsString {
  let mut new_path = join_paths(dirs);
  if !existing_paths.is_empty() {
    new_path.push(":");
    new_path.push(existing_paths);
  }
  new_path
}

fn join_paths(paths: &[&Path]) -> OsString {
  let mut result = OsString::new();
  for path in paths {
    if !result.is_empty() {
      result.push(":");
    }
    result.push(path.as_os_str());
  }
  result
}
pub(crate) fn exit_status_to_code(exit_status: ExitStatus) -> ExitCode {
  if exit_status.success() {
    return ExitCode::SUCCESS;
  }
  let Some(big_code) = exit_status.code() else {
    return ExitCode::FAILURE;
  };
  match u8::try_from(big_code) {
    Ok(small_code) => ExitCode::from(small_code),
    Err(_) => ExitCode::from(255),
  }
}

/// provides a printable version of this `ExecutableCall` when called with additional arguments
pub(crate) fn render_call(executable: &Executable, args: &[String]) -> String {
  let mut result = executable.to_string();
  for arg in args {
    result.push(' ');
    result.push_str(arg);
  }
  result
}

#[cfg(test)]
mod tests {
  use crate::executables::Executable;
  use crate::subshell::render_call;
  use big_s::S;
  use std::ffi::OsString;
  use std::path::Path;

  #[test]
  fn format_with_extra_args() {
    let executable = Executable::from(Path::new("executable"));
    let have = render_call(&executable, &[S("arg1"), S("arg2"), S("arg3")]);
    let want = S("executable arg1 arg2 arg3");
    assert_eq!(have, want);
  }

  #[test]
  fn join_paths() {
    let give = [Path::new("path1"), Path::new("path2")];
    let have = super::join_paths(&give);
    let want = OsString::from("path1:path2");
    assert_eq!(have, want);
  }

  #[test]
  fn prepend_paths() {
    let existing_paths = OsString::from("path1:path2");
    let give = [Path::new("path3"), Path::new("path4")];
    let have = super::prepend_paths(&existing_paths, &give);
    let want = OsString::from("path3:path4:path1:path2");
    assert_eq!(have, want);
  }
}
