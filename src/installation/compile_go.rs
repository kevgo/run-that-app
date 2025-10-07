use super::Outcome;
use crate::configuration::RequestedVersions;
use crate::context::RuntimeContext;
use crate::error::{Result, UserError};
use crate::logging::Event;
use crate::{applications, commands};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;
use which::which;

/// installs the given Go-based application by compiling it from source
pub(crate) fn run(app_folder: &Path, import_path: &str, optional: bool, from_source: bool, ctx: &RuntimeContext) -> Result<Outcome> {
  let go_args = vec!["install", &import_path];
  let go_path = if let Ok(system_go_path) = which("go") {
    system_go_path
  } else {
    let Some(rta_path) = load_rta_go(optional, from_source, ctx)? else {
      return Ok(Outcome::NotInstalled);
    };
    rta_path
  };
  (ctx.log)(Event::CompileGoBegin {
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
    (ctx.log)(Event::CompileGoFailed);
    return Err(UserError::GoCompilationFailed);
  }
  (ctx.log)(Event::CompileGoSuccess);
  Ok(Outcome::Installed)
}

fn load_rta_go(optional: bool, from_source: bool, ctx: &RuntimeContext) -> Result<Option<PathBuf>> {
  use crate::applications::AppDefinition;
  let go = applications::go::Go {};
  let requested_go_versions: RequestedVersions = if let Some(versions) = ctx.config_file.lookup(&go.app_name()) {
    (*versions).clone()
  } else {
    let versions = go.installable_versions(3, ctx.log)?;
    RequestedVersions::from(versions)
  };
  if let Some(executable_call) = commands::run::load_or_install_app(&go, requested_go_versions, optional, from_source, ctx)? {
    return Ok(Some(executable_call.executable.inner()));
  }
  Ok(None)
}
