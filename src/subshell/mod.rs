use std::env;
use std::ffi::OsString;
use std::path::Path;
use std::process::{Command, ExitCode, ExitStatus};

mod capture_output;
mod copy_output;
mod stream_output;

pub(crate) use capture_output::capture_output;
pub(crate) use copy_output::copy_output;
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
