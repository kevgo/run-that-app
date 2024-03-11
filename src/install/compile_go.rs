use crate::apps::App;
use crate::config::Version;
use crate::error::UserError;
use crate::output::Output;
use crate::{yard, Result};
use std::fs;
use std::io::ErrorKind;
use std::process::Command;
use which::which;

/// defines the information needed for RTA to compile a Go app from source
pub trait CompileGo: App {
    fn import_path(&self, version: &Version) -> String;
}

/// installs the given Go-based application by compiling it from source
pub fn run(app: &dyn CompileGo, version: &Version, output: &dyn Output) -> Result<bool> {
    let Ok(go_path) = which("go") else {
        // TODO: install Go and use it to compile this app here
        return Ok(false);
    };
    let yard = yard::load_or_create(&yard::production_location()?)?;
    let target_folder = yard.app_folder(&app.yard_app(), version);
    fs::create_dir_all(&target_folder).map_err(|err| UserError::CannotCreateFolder {
        folder: target_folder.clone(),
        reason: err.to_string(),
    })?;
    let import_path = app.import_path(version);
    let go_args = vec!["install", &import_path];
    output.println(&format!("go {}", go_args.join(" ")));
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
        return Err(UserError::GoCompilationFailed);
    }
    Ok(true)
}
