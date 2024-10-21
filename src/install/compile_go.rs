use crate::apps::{self, App};
use crate::cmd;
use crate::config::Version;
use crate::logger::{Event, Log};
use crate::prelude::*;
use crate::yard::Yard;
use big_s::S;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;
use which::which;

/// defines the information needed to compile a Go app from source
#[allow(clippy::module_name_repetitions)]
pub trait CompileGoSource: App {
  /// the Go import path of the application to compile from source
  fn import_path(&self, version: &Version) -> String;
}

/// installs the given Go-based application by compiling it from source
pub fn run(app: &dyn CompileGoSource, version: &Version, yard: &Yard, log: Log) -> Result<bool> {
  if let Ok(system_go_path) = which("go") {
    compile_using_system_go(system_go_path, app, version, yard, log)
  } else {
    compile_using_rta_go(app, version, yard, log)
  }
}

fn compile_using_system_go(go_path: PathBuf, app: &dyn CompileGoSource, version: &Version, yard: &Yard, log: Log) -> Result<bool> {
  let target_folder = yard.create_app_folder(&app.name(), version)?;
  let import_path = app.import_path(version);
  let go_args = vec!["install", &import_path];
  log(Event::CompileGoBegin {
    go_path: go_path.to_string_lossy(),
    args: &go_args,
  });
  let mut cmd = Command::new(go_path);
  cmd.args(go_args);
  cmd.env("GOBIN", target_folder);
  let status = match cmd.status() {
    Ok(status) => status,
    Err(err) => match err.kind() {
      ErrorKind::PermissionDenied => return Err(UserError::GoNoPermission),
      ErrorKind::Interrupted => return Err(UserError::CompilationInterupted),
      _ => return Err(UserError::CompilationError { reason: err.to_string() }),
    },
  };
  if !status.success() {
    log(Event::CompileGoFailed);
    return Err(UserError::GoCompilationFailed);
  }
  log(Event::CompileGoSuccess);
  Ok(true)
}

fn compile_using_rta_go(app: &dyn CompileGoSource, version: &Version, yard: &Yard, log: Log) -> Result<bool> {
  let go = apps::go::Go {};
  let import_path = app.import_path(version);
  cmd::run(cmd::run::Args {
    app: go.name(),
    version: None,
    app_args: vec![S("install"), import_path],
    error_on_output: false,
    optional: false,
    verbose: false,
  });
  // install RTA Go if it doesn't exist
  // get RTA Go executable path
  // run "go install" using the RTA Go
  Ok(false)
}
