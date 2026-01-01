use std::path::Path;

/// Makes a file executable by setting appropriate permissions
pub(crate) fn set_executable_bit(filepath: &Path) {
  #[cfg(unix)]
  return set_executable_bit_unix(filepath);
  #[cfg(windows)]
  return set_executable_bit_windows(filepath);
}

#[cfg(windows)]
#[allow(clippy::unnecessary_wraps)]
fn set_executable_bit_windows(_filepath: &Path) -> Result<()> {
  // Windows does not have file permissions --> nothing to do here
  Ok(())
}

#[cfg(unix)]
fn set_executable_bit_unix(filepath: &Path) {
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
