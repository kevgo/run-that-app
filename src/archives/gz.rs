use super::Archive;
use crate::error::{Result, UserError};
use crate::logging::{Event, Log};
use flate2::read::GzDecoder;
use std::io;
use std::path::Path;

/// a .tar.gz file downloaded from the internet, containing an application
pub(crate) struct Gz {
  pub(crate) data: Vec<u8>,
}

impl Archive for Gz {
  fn extract_all(&self, target_dir: &Path, log: Log) -> Result<()> {
    log(Event::ArchiveExtractBegin { archive_type: "gz" });
    let gz_decoder = GzDecoder::new(io::Cursor::new(&self.data));
    match gz_decoder.unpack(target_dir) {
      Ok(()) => {
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
