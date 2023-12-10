use crate::error::UserError;
use crate::yard::Executable;
use crate::{Output, Result};
use std::path::PathBuf;
use std::process::Command;
use which::which;

/// installs the given Rust-based application by compiling it from source
pub fn compile_rust(args: CompileArgs) -> Result<Option<Executable>> {
    let Ok(cargo_path) = which("cargo") else {
        return Err(UserError::RustNotInstalled);
    };
    let mut cmd = Command::new(cargo_path);
    cmd.arg("install");
    cmd.arg("--root");
    cmd.arg(&args.target_folder);
    cmd.arg("--locked");
    cmd.arg(args.crate_name);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => return Err(UserError::RustNotInstalled),
            std::io::ErrorKind::PermissionDenied => return Err(UserError::RustNoPermission),
            std::io::ErrorKind::Interrupted => return Ok(None),
            _ => panic!("{}", err.to_string()),
        },
    };
    if !status.success() {
        return Err(UserError::RustCompilationFailed);
    }
    let executable = Executable(args.target_folder.join(args.executable_filename));
    drop(args);
    Ok(Some(executable))
}

pub struct CompileArgs<'a> {
    pub crate_name: &'static str,
    pub target_folder: PathBuf,
    pub executable_filename: &'static str,
    pub output: &'a dyn Output,
}
