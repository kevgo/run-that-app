//! This module implements various ways to execute work in subshells.

use std::env;
use std::ffi::{OsStr, OsString};
use std::path::Path;
use std::process::{Command, ExitCode, ExitStatus};

mod capture_output;
mod detect_output;
mod shellscript;
mod stream_output;

pub use capture_output::capture_output;
pub use detect_output::detect_output;
pub use shellscript::shell_script_call;
pub use stream_output::stream_output;

/// adds the given dirs to the PATH env variable of the given cmd
pub fn add_paths(cmd: &mut Command, dirs: &[&Path]) {
  cmd.envs(env::vars_os());
  set_path_env(cmd, join_path_expressions(&join_paths(dirs), &path_env_value()));
}

pub fn path_expressions(dirs: &[&Path]) -> OsString {
  join_path_expressions(&join_paths(dirs), &path_env_value())
}

/// Sets PATH on the command using the same environment-variable casing as this process.
///
/// `std::process::Command` treats env var names as case-sensitive even on Windows,
/// where the existing variable is typically `Path`. Setting `PATH` would leave the
/// original `Path` in place and the child process would not see our directories.
pub fn set_path_env(cmd: &mut Command, path: impl AsRef<OsStr>) {
  cmd.env(path_env_key(), path);
}

fn path_env_key() -> OsString {
  env::vars_os()
    .map(|(key, _)| key)
    .find(|key| key.eq_ignore_ascii_case("PATH"))
    .unwrap_or_else(|| OsString::from("PATH"))
}

fn path_env_value() -> OsString {
  env::vars_os()
    .find(|(key, _)| key.eq_ignore_ascii_case("PATH"))
    .map(|(_, value)| value)
    .unwrap_or_default()
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

pub fn exit_status_to_code(exit_status: ExitStatus) -> ExitCode {
  if exit_status.success() {
    return ExitCode::SUCCESS;
  }
  let Some(big_code) = exit_status.code() else {
    return ExitCode::FAILURE;
  };
  ExitCode::from(u8::try_from(big_code).unwrap_or(255))
}

#[cfg(test)]
mod tests {

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
      #[cfg(windows)]
      let want = OsString::from("path1;path2");
      #[cfg(not(windows))]
      let want = OsString::from("path1:path2");
      assert_eq!(have, want);
    }

    #[test]
    fn three() {
      let give = [Path::new("path1"), Path::new("path2"), Path::new("path3")];
      let have = super::super::join_paths(&give);
      #[cfg(windows)]
      let want = OsString::from("path1;path2;path3");
      #[cfg(not(windows))]
      let want = OsString::from("path1:path2:path3");
      assert_eq!(have, want);
    }
  }

  mod join_path_expressions {
    use std::ffi::OsString;

    #[test]
    #[cfg(windows)]
    fn both_non_empty() {
      let first = OsString::from("path1;path2");
      let second = OsString::from("path3;path4");
      let have = super::super::join_path_expressions(&first, &second);
      let want = OsString::from("path1;path2;path3;path4");
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(not(windows))]
    fn both_non_empty() {
      let first = OsString::from("path1:path2");
      let second = OsString::from("path3:path4");
      let have = super::super::join_path_expressions(&first, &second);
      let want = OsString::from("path1:path2:path3:path4");
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn first_empty() {
      let first = OsString::from("");
      let second = OsString::from("path3;path4");
      let have = super::super::join_path_expressions(&first, &second);
      let want = OsString::from("path3;path4");
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(not(windows))]
    fn first_empty() {
      let first = OsString::from("");
      let second = OsString::from("path3:path4");
      let have = super::super::join_path_expressions(&first, &second);
      let want = OsString::from("path3:path4");
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn second_empty() {
      let first = OsString::from("path1;path2");
      let second = OsString::from("");
      let have = super::super::join_path_expressions(&first, &second);
      let want = OsString::from("path1;path2");
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(not(windows))]
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

  mod path_env_key {
    #[test]
    fn matches_existing_path_variable_casing() {
      let key = super::super::path_env_key();
      assert!(key.eq_ignore_ascii_case("PATH"));
      let found = std::env::vars_os().any(|(existing, _)| existing == key);
      assert!(found || key == "PATH");
    }
  }
}
