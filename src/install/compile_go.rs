use super::Outcome;
use crate::apps::{self, App};
use crate::config::{RequestedVersion, RequestedVersions, Version};
use crate::logger::{Event, Log};
use crate::platform::Platform;
use crate::prelude::*;
use crate::yard::Yard;
use crate::{cmd, config};
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
pub fn run(app: &dyn CompileGoSource, platform: Platform, version: &Version, config_file: &config::File, yard: &Yard, log: Log) -> Result<Outcome> {
  let target_folder = yard.create_app_folder(&app.name(), version)?;
  let import_path = app.import_path(version);
  let go_args = vec!["install", &import_path];
  let go_path = if let Ok(system_go_path) = which("go") {
    system_go_path
  } else {
    let Some(rta_path) = load_rta_go(platform, config_file, yard, log)? else {
      return Ok(Outcome::NotInstalled);
    };
    rta_path
  };
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

fn load_rta_go(platform: Platform, config_file: &config::File, yard: &Yard, log: Log) -> Result<Option<PathBuf>> {
  let go = apps::go::Go {};
  let requested_go_versions = if let Some(versions) = config_file.lookup(&go.name()) {
    versions
  } else {
    let versions = go.installable_versions(3, log)?;
    &RequestedVersions::new(versions.into_iter().map(RequestedVersion::from).collect())
  };
  for requested_go_version in &requested_go_versions.0 {
    if let Some(executable) = cmd::run::load_or_install(&go, requested_go_version, platform, yard, config_file, log)? {
      return Ok(Some(executable.0));
    }
  }
  Ok(None)
}
