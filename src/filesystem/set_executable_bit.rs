use crate::error::Result;
use std::path::Path;

/// Makes a file executable by setting appropriate permissions
pub(crate) fn set_executable_bit(filepath: &Path) -> Result<()> {
  #[cfg(unix)]
  return set_executable_bit_unix(filepath);
  #[cfg(windows)]
  return set_executable_bit_windows(filepath, log);
}

#[cfg(windows)]
fn set_executable_bit_windows(_filepath: &Path, _log: Log) -> Result<()> {
  // Windows does not have file permissions --> nothing to do here
  Ok(())
}

#[cfg(unix)]
fn set_executable_bit_unix(filepath: &Path) -> Result<()> {
  use crate::error::UserError;
  use std::fs;
  use std::os::unix::fs::PermissionsExt;
  let Ok(executable_file) = fs::File::open(filepath) else {
    return Err(UserError::ArchiveDoesNotContainExecutable {
      expected: filepath.to_path_buf(),
    });
  };
  let metadata = match executable_file.metadata() {
    Ok(metadata) => metadata,
    Err(err) => {
      return Err(UserError::CannotReadFileMetadata { err: err.to_string() });
    }
  };
  let mut permissions = metadata.permissions();
  if permissions.mode() & 0o100 != 0 {
    // file is already executable
    return Ok(());
  }
  permissions.set_mode(0o744);
  fs::set_permissions(filepath, permissions).map_err(|err| UserError::CannotSetFilePermissions {
    path: filepath.to_path_buf(),
    err: err.to_string(),
  })
}
