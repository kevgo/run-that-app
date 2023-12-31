use crate::error::UserError;
use crate::output::Output;
use crate::yard::Executable;
use crate::Result;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use which::which;

/// installs the given Go-based application by compiling it from source
pub fn compile_go(args: CompileArgs) -> Result<Option<Executable>> {
    let Ok(go_path) = which("go") else {
        return Err(UserError::GoNotInstalled);
    };
    fs::create_dir_all(&args.target_folder).map_err(|err| UserError::CannotCreateFolder {
        folder: args.target_folder.clone(),
        reason: err.to_string(),
    })?;
    let go_args = vec!["install", &args.import_path];
    args.output.println(&format!("go {}", go_args.join(" ")));
    let mut cmd = Command::new(go_path);
    cmd.args(go_args);
    cmd.env("GOBIN", &args.target_folder);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => return Err(UserError::GoNotInstalled),
            std::io::ErrorKind::PermissionDenied => return Err(UserError::GoNoPermission),
            std::io::ErrorKind::Interrupted => return Ok(None),
            _ => panic!("{}", err.to_string()),
        },
    };
    if !status.success() {
        return Err(UserError::GoCompilationFailed);
    }
    let executable = Executable(args.target_folder.join(args.executable_filename));
    drop(args);
    Ok(Some(executable))
}

pub struct CompileArgs<'a> {
    /// the fully qualified Go import path for the package to install
    pub import_path: String,
    pub target_folder: PathBuf,
    pub executable_filename: &'static str,
    pub output: &'a dyn Output,
}
