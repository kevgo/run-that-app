use crate::error::UserError;
use crate::output::Output;
use crate::yard::Executable;
use crate::Result;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use which::which;

pub fn compile_from_go_source(args: CompileFromGoSource, output: &dyn Output) -> Result<Option<Executable>> {
    let Ok(go_path) = which("go") else {
        return Err(UserError::GoNotInstalled);
    };
    fs::create_dir_all(&args.target_folder).map_err(|err| UserError::CannotCreateFolder {
        folder: args.target_folder.clone(),
        reason: err.to_string(),
    })?;
    output.println(&format!("go install {}", &args.import_path));
    let mut cmd = Command::new(go_path);
    cmd.arg("install");
    cmd.arg(&args.import_path);
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
    Ok(Some(Executable(args.target_folder.join(args.executable_filename))))
}

pub struct CompileFromGoSource {
    /// the fully qualified Go import path for the package to install
    pub import_path: String,
    pub target_folder: PathBuf,
    pub executable_filename: &'static str,
}
