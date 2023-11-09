use crate::error::UserError;
use crate::Result;
use std::fs;
use std::path::Path;

#[cfg(unix)]
pub fn make_file_executable(file: &Path) -> Result<()> {
    use std::os::unix::prelude::PermissionsExt;
    fs::set_permissions(file, fs::Permissions::from_mode(0o744)).map_err(|err| {
        UserError::CannotMakeFileExecutable {
            file: file.to_string_lossy().to_string(),
            reason: err.to_string(),
        }
    })
}

#[cfg(not(unix))]
pub fn make_executable(file: &Path) -> Result<()> {
    Ok(())
}
