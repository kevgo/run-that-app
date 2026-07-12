use super::Outcome;
use crate::applications;
use crate::applications::Apps;
use crate::context::RuntimeContext;
use crate::error::{Result, UserError};
use crate::executables::{LoadOrInstallAppWithCarrierOutcome, load_or_install_app_and_carrier};
use crate::logging::Event;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;
use which::which;

/// installs the given Go-based application by compiling it from source
pub fn run(app_folder: &Path, import_path: &str, optional: bool, ctx: &RuntimeContext, apps: &Apps) -> Result<Outcome> {
  let go_args = vec!["install", &import_path];
  let go_path = if let Ok(system_go_path) = which("go") {
    system_go_path
  } else {
    let Some(rta_path) = load_rta_go(optional, ctx, apps)? else {
      return Ok(Outcome::NotInstalled { app: "go".into() });
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

fn load_rta_go(optional: bool, ctx: &RuntimeContext, apps: &Apps) -> Result<Option<PathBuf>> {
  let go = applications::Go {};
  match load_or_install_app_and_carrier(&go, None, optional, false, ctx, apps)? {
    LoadOrInstallAppWithCarrierOutcome::Loaded { executable_call } => Ok(Some(executable_call.executable.into())),
    LoadOrInstallAppWithCarrierOutcome::NotInstallable { app: _ } => Ok(None),
  }
}
