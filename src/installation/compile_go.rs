use super::Outcome;
use crate::applications::{self, AppDefinition};
use crate::configuration::{RequestedVersion, RequestedVersions};
use crate::logging::{Event, Log};
use crate::platform::Platform;
use crate::prelude::*;
use crate::yard::Yard;
use crate::{commands, configuration};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;
use which::which;

/// installs the given Go-based application by compiling it from source
pub(crate) fn run(
  app_folder: &Path,
  import_path: &str,
  platform: Platform,
  optional: bool,
  config_file: &configuration::File,
  yard: &Yard,
  log: Log,
) -> Result<Outcome> {
  let go_args = vec!["install", &import_path];
  let go_path = if let Ok(system_go_path) = which("go") {
    system_go_path
  } else {
    let Some(rta_path) = load_rta_go(platform, optional, config_file, yard, log)? else {
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
  cmd.env("GOBIN", app_folder);
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

fn load_rta_go(platform: Platform, optional: bool, config_file: &configuration::File, yard: &Yard, log: Log) -> Result<Option<PathBuf>> {
  let go = applications::go::Go {};
  let requested_go_versions = if let Some(versions) = config_file.lookup(&go.app_name()) {
    versions
  } else {
    let versions = go.installable_versions(3, log)?;
    &RequestedVersions::new(versions.into_iter().map(RequestedVersion::from).collect())
  };
  for requested_go_version in &requested_go_versions.0 {
    if let Some(executable_call) = commands::run::load_or_install(&go, requested_go_version, platform, optional, yard, config_file, log)? {
      return Ok(Some(executable_call.executable_path.inner()));
    }
  }
  Ok(None)
}
