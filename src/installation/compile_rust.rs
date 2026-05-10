use super::Outcome;
use crate::applications::AppDefinition;
use crate::configuration::Version;
use crate::download::Url;
use crate::error::{Result, UserError};
use crate::logging::{Event, Log};
use big_s::S;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;
use which::which;

/// the different locations from where to get Rust source code
pub(crate) enum RustSource {
  /// install from crates.io
  CratesIo { name: &'static str },
  /// install from a remote repository
  Repository { url: Url },
}

/// installs the given Rust-based application by compiling it from source
pub(crate) fn run(app_definition: &dyn AppDefinition, version: &Version, app_folder: &Path, source: &RustSource, log: Log) -> Result<Outcome> {
  let Ok(cargo_path) = which("cargo") else {
    return Err(UserError::RustNotInstalled);
  };
  let mut cmd = Command::new(&cargo_path);
  let app_folder_str = app_folder.to_string_lossy().to_string();
  let mut args: Vec<String> = vec![S("install"), S("--root"), app_folder_str, S("--locked")];
  match &source {
    RustSource::CratesIo { name } => args.push(S(name)),
    RustSource::Repository { url } => {
      args.push(S("--git"));
      args.push(url.to_string());
      args.push(S("--tag"));
      args.push(app_definition.tag_format().format_version(version));
    }
  }
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
