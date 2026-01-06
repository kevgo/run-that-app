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

// TODO: on Windows, separate the paths with a semicolon instead of a colon

/// adds the given dirs to the PATH env variable of the given cmd
pub(crate) fn add_paths(cmd: &mut Command, dirs: &[&Path]) {
  cmd.envs(env::vars_os());
  let new_path = join_path_expressions(&join_paths(dirs), &env::var_os("PATH").unwrap_or_default());
  println!(
    "2222222222222222222222222222222222222222222222222222222222222222 {}",
    new_path.to_string_lossy()
  );
  cmd.env("PATH", new_path);
}

/// joins the given PATH expressions (containing multiple paths) into a single PATH expression
fn join_path_expressions(first: &OsString, second: &OsString) -> OsString {
  let mut new_path = OsString::with_capacity(first.len() + second.len() + 1);
  if !first.is_empty() {
    new_path.push(first);
  }
  if !second.is_empty() {
    if !new_path.is_empty() {
      new_path.push(paths_separator());
    }
    new_path.push(second);
  }
  new_path
}

/// joins the given paths into a single PATH expression
fn join_paths(paths: &[&Path]) -> OsString {
  let mut result = OsString::new();
  for path in paths {
    if !result.is_empty() {
      result.push(paths_separator());
    }
    result.push(path.as_os_str());
  }
  result
}

fn paths_separator() -> &'static str {
  if cfg!(windows) { ";" } else { ":" }
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
  use std::path::Path;

  #[test]
  fn format_with_extra_args() {
    let executable = Executable::from(Path::new("executable"));
    let have = render_call(&executable, &[S("arg1"), S("arg2"), S("arg3")]);
    let want = S("executable arg1 arg2 arg3");
    assert_eq!(have, want);
  }

  mod join_paths {
    use std::ffi::OsString;
    use std::path::Path;

    #[test]
    fn zero() {
      let give = [];
      let have = super::super::join_paths(&give);
      let want = OsString::from("");
      assert_eq!(have, want);
    }

    #[test]
    fn one() {
      let give = [Path::new("path1")];
      let have = super::super::join_paths(&give);
      let want = OsString::from("path1");
      assert_eq!(have, want);
    }

    #[test]
    fn two() {
      let give = [Path::new("path1"), Path::new("path2")];
      let have = super::super::join_paths(&give);
      let want = OsString::from("path1:path2");
      assert_eq!(have, want);
    }

    #[test]
    fn three() {
      let give = [Path::new("path1"), Path::new("path2"), Path::new("path3")];
      let have = super::super::join_paths(&give);
      let want = OsString::from("path1:path2:path3");
      assert_eq!(have, want);
    }
  }

  mod join_path_expressions {
    use std::ffi::OsString;

    #[test]
    fn both_non_empty() {
      let first = OsString::from("path1:path2");
      let second = OsString::from("path3:path4");
      let have = super::super::join_path_expressions(&first, &second);
      let want = OsString::from("path1:path2:path3:path4");
      assert_eq!(have, want);
    }

    #[test]
    fn first_empty() {
      let first = OsString::from("");
      let second = OsString::from("path3:path4");
      let have = super::super::join_path_expressions(&first, &second);
      let want = OsString::from("path3:path4");
      assert_eq!(have, want);
    }

    #[test]
    fn second_empty() {
      let first = OsString::from("path1:path2");
      let second = OsString::from("");
      let have = super::super::join_path_expressions(&first, &second);
      let want = OsString::from("path1:path2");
      assert_eq!(have, want);
    }

    #[test]
    fn both_empty() {
      let first = OsString::from("");
      let second = OsString::from("");
      let have = super::super::join_path_expressions(&first, &second);
      let want = OsString::from("");
      assert_eq!(have, want);
    }
  }
}
