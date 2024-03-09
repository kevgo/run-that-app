use crate::config::Version;
use crate::error::UserError;
use crate::output::Output;
use crate::Result;
use std::any::Any;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;
use which::which;

pub trait CompileFromGoSource {
    fn import_path(&self, version: &Version) -> String;
}

/// installs the given Go-based application by compiling it from source
pub fn compile_go(app: &dyn CompileFromGoSource, version: &Version, output: &dyn Output) -> Result<bool> {
    let Ok(go_path) = which("go") else {
        return Ok(false);
    };
    let target_folder =  app.type_id()
    fs::create_dir_all(args.target_folder).map_err(|err| UserError::CannotCreateFolder {
        folder: args.target_folder.to_path_buf(),
        reason: err.to_string(),
    })?;
    let go_args = vec!["install", &args.import_path];
    args.output.println(&format!("go {}", go_args.join(" ")));
    let mut cmd = Command::new(go_path);
    cmd.args(go_args);
    cmd.env("GOBIN", args.target_folder);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(err) => match err.kind() {
            ErrorKind::PermissionDenied => return Err(UserError::GoNoPermission),
            ErrorKind::Interrupted => return Err(UserError::CompilationInterupted),
            other => return Err(UserError::CompilationError { reason: err.to_string() }),
        },
    };
    if !status.success() {
        return Err(UserError::GoCompilationFailed);
    }
    Ok(true)
}

pub struct CompileArgs<'a> {
    /// the fully qualified Go import path for the package to install
    pub import_path: String,
    pub target_folder: &'a Path,
    pub output: &'a dyn Output,
}
