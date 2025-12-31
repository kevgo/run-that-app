mod gz;
mod tar_gz;
mod tar_xz;
mod zip;

use self::gz::Gz;
use self::tar_gz::TarGz;
use self::tar_xz::TarXz;
use self::zip::Zip;
use crate::error::Result;
use crate::{Log, filesystem};
use std::path::Path;

/// An archive is a compressed file containing an executable and other files needed to run a particular application.
pub(crate) trait Archive {
  /// extracts all files from the given archive data to the given location on disk
  fn extract_all(&self, target_dir: &Path, log: Log) -> Result<()>;
}

/// provides the archive that can extract the given file path
pub(crate) fn lookup(filepath: &str, data: Vec<u8>) -> Option<Box<dyn Archive>> {
  match () {
    () if filesystem::has_extension(filepath, ".tar.gz") => Some(Box::new(TarGz { data })),
    () if filesystem::has_extension(filepath, ".tar.xz") => Some(Box::new(TarXz { data })),
    () if filesystem::has_extension(filepath, ".zip") => Some(Box::new(Zip { data })),
    () if filesystem::has_extension(filepath, ".gz") => Some(Box::new(Gz {
      data,
      filename: filepath.to_string(),
    })),
    () => None,
  }
}

/// Makes a file executable by setting appropriate permissions
pub(crate) fn make_executable(filepath: &Path, log: Log) -> Result<()> {
  #[cfg(unix)]
  return make_executable_unix(filepath, log);
  #[cfg(windows)]
  return make_executable_windows(filepath, log);
}

#[cfg(windows)]
fn make_executable_windows(_filepath: &Path, _log: Log) -> Result<()> {
  // Windows does not have file permissions --> nothing to do here
  Ok(())
}

#[cfg(unix)]
fn make_executable_unix(filepath: &Path, log: Log) -> Result<()> {
  use crate::error::UserError;
  use crate::logging::Event;
  use std::fs;
  use std::os::unix::fs::PermissionsExt;

  log(Event::MakeExecutable { file: filepath });
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

#[cfg(test)]
mod tests {

  mod lookup {
    use crate::archives::lookup;

    #[test]
    fn known_archive_type() {
      let have = lookup("archive.zip", vec![]);
      assert!(have.is_some());
    }

    #[test]
    fn unknown_archive_type() {
      let have = lookup("archive.zonk", vec![]);
      assert!(have.is_none());
    }
  }
}
