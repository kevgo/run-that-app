use std::path::Path;

/// Makes the given file executable by setting appropriate permissions.
/// Ignores non-existing files.
#[cfg(not(windows))]
pub fn set_executable_bit(filepath: &Path) {
  use std::fs;
  use std::os::unix::fs::PermissionsExt;
  let Ok(executable_file) = fs::File::open(filepath) else { return };
  let Ok(metadata) = executable_file.metadata() else { return };
  let mut permissions = metadata.permissions();
  if permissions.mode() & 0o100 != 0 {
    // file is already executable
    return;
  }
  permissions.set_mode(0o744);
  let _ = fs::set_permissions(filepath, permissions);
}

/// Does nothing on Windows since Windows determines executability through file extensions.
#[cfg(windows)]
pub fn set_executable_bit(_filepath: &Path) {}
