use super::Archive;
use crate::error::{Result, UserError};
use crate::logging::{Event, Log};
use flate2::read::GzDecoder;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

/// a .gz file downloaded from the internet, containing a single executable
pub(crate) struct Gz {
  pub(crate) data: Vec<u8>,
  pub(crate) filename: String,
}

impl Archive for Gz {
  fn extract_all(&self, target_dir: &Path, log: Log) -> Result<()> {
    log(Event::ArchiveExtractBegin { archive_type: "gz" });

    // determine output filename by removing .gz extension
    let filename_path = PathBuf::from(&self.filename);
    let output_filename = filename_path
      .file_name()
      .and_then(|name| name.to_str())
      .and_then(|name| name.strip_suffix(".gz"))
      .ok_or_else(|| UserError::ArchiveCannotExtract {
        reason: format!("Cannot determine output filename from: {}", self.filename),
      })?;

    let output_path = target_dir.join(output_filename);

    // stream decompressed data directly to file
    let mut gz_decoder = GzDecoder::new(io::Cursor::new(&self.data));
    match File::create(&output_path) {
      Ok(mut file) => {
        if let Err(err) = io::copy(&mut gz_decoder, &mut file) {
          log(Event::ArchiveExtractFailed { err: err.to_string() });
          return Err(UserError::ArchiveCannotExtract { reason: err.to_string() });
        }
        drop(file); // close file before setting permissions
        make_executable(&output_path, log)?;
        log(Event::ArchiveExtractSuccess);
        Ok(())
      }
      Err(err) => {
        log(Event::ArchiveExtractFailed { err: err.to_string() });
        Err(UserError::ArchiveCannotExtract { reason: err.to_string() })
      }
    }
  }
}

#[cfg(unix)]
fn make_executable(filepath: &Path, log: Log) -> Result<()> {
  use std::os::unix::fs::PermissionsExt;

  log(Event::MakeExecutable { file: filepath });
  let metadata = fs::metadata(filepath).map_err(|err| UserError::CannotReadFileMetadata { err: err.to_string() })?;
  let mut permissions = metadata.permissions();
  if permissions.mode() & 0o100 == 0 {
    permissions.set_mode(0o744);
    fs::set_permissions(filepath, permissions).map_err(|err| UserError::CannotSetFilePermissions {
      path: filepath.to_path_buf(),
      err: err.to_string(),
    })?;
  }
  Ok(())
}

#[cfg(windows)]
fn make_executable(_filepath: &Path, _log: Log) -> Result<()> {
  // Windows does not have file permissions --> nothing to do here
  Ok(())
}
