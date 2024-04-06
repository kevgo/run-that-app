use crate::apps::App;
use crate::config::Version;
use crate::logger::{Event, Log};
use crate::prelude::*;
use crate::yard::Yard;
use std::process::Command;
use which::which;

/// defines the information needed to compile a Rust app from source
#[allow(clippy::module_name_repetitions)]
pub trait CompileRustSource: App {
  /// the name of the Rust crate containing the source code of the application to compile
  fn crate_name(&self) -> &'static str;
}

/// installs the given Rust-based application by compiling it from source
pub fn run(app: &dyn CompileRustSource, version: &Version, yard: &Yard, log: Log) -> Result<bool> {
  let Ok(cargo_path) = which("cargo") else {
    return Err(UserError::RustNotInstalled);
  };
  let target_folder = yard.create_app_folder(&app.name(), version)?;
  let mut cmd = Command::new(&cargo_path);
  let target_folder_str = &target_folder.to_string_lossy();
  let args = vec!["install", "--root", &target_folder_str, "--locked", app.crate_name()];
  log(Event::CompileRustStart {
    cargo_path: &cargo_path,
    args: &args,
  });
  cmd.args(args);
  let status = match cmd.status() {
    Ok(status) => status,
    Err(err) => match err.kind() {
      std::io::ErrorKind::NotFound => return Err(UserError::RustNotInstalled),
      std::io::ErrorKind::PermissionDenied => return Err(UserError::RustNoPermission),
      std::io::ErrorKind::Interrupted => return Ok(false),
      _ => panic!("{}", err.to_string()),
    },
  };
  if !status.success() {
    log(Event::CompileRustFailed);
    return Err(UserError::RustCompilationFailed);
  }
  log(Event::CompileRustSuccess);
  Ok(true)
}
