use crate::apps::App;
use crate::config::Version;
use crate::error::UserError;
use crate::logger::{Event, Log};
use crate::yard::Yard;
use crate::Result;
use std::io::ErrorKind;
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
    let Ok(go_path) = which("go") else {
        return Ok(false);
    };
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
