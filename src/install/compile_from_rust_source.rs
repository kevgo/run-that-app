use super::InstallationMethod;
use crate::error::UserError;
use crate::yard::Executable;
use std::path::PathBuf;
use std::process::Command;
use which::which;

pub struct CompileFromRustSource {
    pub crate_name: &'static str,
    pub target_folder: PathBuf,
    pub executable_filename: &'static str,
}

impl InstallationMethod for CompileFromRustSource {
    fn install(&self, _output: &dyn crate::output::Output) -> crate::error::Result<Option<crate::yard::Executable>> {
        let Ok(cargo_path) = which("cargo") else {
            return Err(UserError::RustNotInstalled);
        };
        let mut cmd = Command::new(cargo_path);
        cmd.arg("install");
        cmd.arg("--root");
        cmd.arg(&self.target_folder);
        cmd.arg("--locked");
        cmd.arg(self.crate_name);
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
        Ok(Some(Executable(self.target_folder.join(self.executable_filename))))
    }
}
