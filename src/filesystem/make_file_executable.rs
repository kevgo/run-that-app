use crate::Result;
use std::path::Path;

#[cfg(unix)]
pub fn make_file_executable(file: &Path) -> Result<()> {
    use crate::error::UserError;
    use std::fs;
    use std::os::unix::prelude::PermissionsExt;
    fs::set_permissions(file, fs::Permissions::from_mode(0o744)).map_err(|err| UserError::CannotMakeFileExecutable {
        file: file.to_string_lossy().to_string(),
        reason: err.to_string(),
    })
}

#[cfg(windows)]
#[allow(clippy::unnecessary_wraps)]
pub fn make_file_executable(_file: &Path) -> Result<()> {
    Ok(())
}
