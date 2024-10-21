use super::Outcome;
use crate::apps::go::Go;
use crate::apps::{self, App};
use crate::config::{AppName, RequestedVersion, RequestedVersions, Version};
use crate::logger::{Event, Log};
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{cmd, config};
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
pub fn run(app: &dyn CompileGoSource, platform: Platform, version: &Version, yard: &Yard, log: Log) -> Result<Outcome> {
  if let Ok(system_go_path) = which("go") {
    compile_using_system_go(system_go_path, app, version, yard, log)
  } else {
    compile_using_rta_go(app, platform, version, yard, log)
  }
}

fn compile_using_system_go(go_path: PathBuf, app: &dyn CompileGoSource, version: &Version, yard: &Yard, log: Log) -> Result<Outcome> {
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
  Ok(Outcome::Installed)
}

fn compile_using_rta_go(app: &dyn CompileGoSource, platform: Platform, app_version: &Version, config: config::File, yard: &Yard, log: Log) -> Result<Outcome> {
  let target_folder = yard.create_app_folder(&app.name(), app_version)?;
  // get the Go version to use
  let go = apps::go::Go {};
  let requested_go_versions = match config.lookup(&go.name()) {
    Some(versions) => versions,
    None => {
      let versions = go.installable_versions(5, log)?;
      RequestedVersions::new(versions.into_iter().map(RequestedVersion::from).collect())
    }
  };
  // get the executable, install Go if needed
  let Some(go_executable) = load_or_install_go(&go, requested_go_versions, platform, yard, log)? else {
    return Ok(Outcome::NotInstalled);
  };
  let import_path = app.import_path(app_version);
  let mut cmd = Command::new(go_executable.0);
  let go_args = vec!["install", &import_path];
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
  Ok(Outcome::Installed)
}

fn load_or_install_go(go: &Go, requested_go_versions: RequestedVersions, platform: Platform, yard: &Yard, log: Log) -> Result<Option<Executable>> {
  for requested_go_version in requested_go_versions {
    if let Some(executable) = cmd::run::load_or_install(go, &requested_go_version, platform, yard, log)? {
      return Ok(Some(executable));
    }
  }
  Ok(None)
}
