use super::Archive;
use crate::error::{Result, UserError};
use crate::logging::{Event, Log};
use flate2::read::GzDecoder;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

/// a .gz file downloaded from the internet, containing a single executable
pub(crate) struct Gz {
  pub(crate) data: Vec<u8>,
  pub(crate) filename: String,
}

impl Archive for Gz {
  fn extract_all(&self, target_dir: &Path, log: Log) -> Result<()> {
    log(Event::ArchiveExtractBegin { archive_type: "gz" });

    // Decompress the gzip data
    let mut gz_decoder = GzDecoder::new(io::Cursor::new(&self.data));
    let mut decompressed_data = Vec::new();

    if let Err(err) = gz_decoder.read_to_end(&mut decompressed_data) {
      log(Event::ArchiveExtractFailed { err: err.to_string() });
      return Err(UserError::ArchiveCannotExtract { reason: err.to_string() });
    }

    // Determine output filename by removing .gz extension
    let filename_path = PathBuf::from(&self.filename);
    let output_filename = filename_path
      .file_name()
      .and_then(|name| name.to_str())
      .and_then(|name| name.strip_suffix(".gz"))
      .ok_or_else(|| UserError::ArchiveCannotExtract {
        reason: format!("Cannot determine output filename from: {}", self.filename),
      })?;

    let output_path = target_dir.join(output_filename);

    // Write decompressed data to file
    match File::create(&output_path) {
      Ok(mut file) => {
        if let Err(err) = file.write_all(&decompressed_data) {
          log(Event::ArchiveExtractFailed { err: err.to_string() });
          return Err(UserError::ArchiveCannotExtract { reason: err.to_string() });
        }
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
