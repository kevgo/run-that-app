use super::Outcome;
use crate::applications::AppDefinition;
use crate::configuration::Version;
use crate::logging::{Event, Log};
use crate::prelude::*;
use crate::yard::Yard;
use std::io::ErrorKind;
use std::process::Command;
use which::which;

/// installs the given Rust-based application by compiling it from source
pub(crate) fn run(app_definition: &dyn AppDefinition, crate_name: &str, version: &Version, yard: &Yard, log: Log) -> Result<Outcome> {
  let Ok(cargo_path) = which("cargo") else {
    return Err(UserError::RustNotInstalled);
  };
  let target_folder = yard.create_app_folder(&app_definition.name(), version)?;
  let mut cmd = Command::new(&cargo_path);
  let target_folder_str = &target_folder.to_string_lossy();
  let args = vec!["install", "--root", &target_folder_str, "--locked", crate_name];
  log(Event::CompileRustStart {
    cargo_path: &cargo_path,
    args: &args,
  });
  cmd.args(args);
  let status = match cmd.status() {
    Ok(status) => status,
    Err(err) => match err.kind() {
      ErrorKind::NotFound => return Err(UserError::RustNotInstalled),
      ErrorKind::PermissionDenied => return Err(UserError::RustNoPermission),
      ErrorKind::Interrupted => return Ok(Outcome::NotInstalled),
      _ => return Err(UserError::CannotCompileRustSource { err: err.to_string() }),
    },
  };
  if !status.success() {
    log(Event::CompileRustFailed);
    return Err(UserError::RustCompilationFailed);
  }
  log(Event::CompileRustSuccess);
  Ok(Outcome::Installed)
}
