use super::InstallationMethod;
use crate::error::UserError;
use crate::output::Output;
use crate::yard::Executable;
use crate::Result;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub struct CompileFromGoSource {
    /// the fully qualified Go import path for the package to install
    pub import_path: String,
    pub target_folder: PathBuf,
    pub executable_filename: &'static str,
}

impl InstallationMethod for CompileFromGoSource {
    fn install(&self, _output: &dyn Output) -> Result<Option<Executable>> {
        let Ok(go_path) = which::which("go") else {
            return Err(UserError::GoNotInstalled);
        };
        fs::create_dir_all(&self.target_folder).map_err(|err| UserError::CannotCreateFolder {
            folder: self.target_folder.to_path_buf(),
            reason: err.to_string(),
        })?;
        let mut cmd = Command::new(go_path);
        cmd.arg("install");
        cmd.arg(&self.import_path);
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
        Ok(Some(Executable(
            self.target_folder.join(self.executable_filename),
        )))
    }
}
