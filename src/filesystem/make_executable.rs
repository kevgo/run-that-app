use crate::error::Result;
#[cfg(unix)]
use crate::error::UserError;
use std::path::Path;

pub(crate) fn make_executable(filepath: &Path) {
  #[cfg(unix)]
  make_executable_unix(filepath)
  #[cfg(windows)]
  make_executable_windows(filepath, log);
}

#[cfg(windows)]
fn make_executable_windows(_filepath: &Path, _log: Log) {
  // Windows does not have file permissions --> nothing to do here
}

#[cfg(unix)]
fn make_executable_unix(filepath: &Path) -> Result<()> {
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
  if permissions.mode() & 0o100 == 0 {
    permissions.set_mode(0o744);
    if let Err(err) = fs::set_permissions(filepath, permissions) {
      return Err(UserError::CannotSetFilePermissions {
        path: filepath.to_path_buf(),
        err: err.to_string(),
      });
    }
  }
  Ok(())
}
