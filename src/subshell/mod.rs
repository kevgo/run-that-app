use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::process::{Command, ExitCode, ExitStatus};

mod capture_output;
mod detect_output;
mod stream_output;

use crate::run::Executable;
pub(crate) use capture_output::capture_output;
pub(crate) use detect_output::detect_output;
pub(crate) use stream_output::stream_output;

/// adds the given dirs to the PATH env variable of the given cmd
pub(crate) fn add_paths(cmd: &mut Command, dirs: &[&Path]) {
  cmd.envs(env::vars_os());
  let new_path = if let Some(mut path) = env::var_os("PATH") {
    // PATH env var is set to something here, could be empty string
    for dir in dirs {
      if !path.is_empty() {
        path.push(":");
      }
      path.push(dir.as_os_str());
    }
    path
  } else {
    // PATH env var is empty here
    let mut path = OsString::new();
    for dir in dirs {
      if !path.is_empty() {
        path.push(":");
      }
      path.push(dir);
    }
    path
  };
  cmd.env("PATH", new_path);
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
  use crate::run::Executable;
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
}
