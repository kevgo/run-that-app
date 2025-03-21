use super::Archive;
use crate::logging::{Event, Log};
use crate::prelude::*;
use std::io::Cursor;
use std::path::Path;
use xz2::read::XzDecoder;

/// a .tar.xz file downloaded from the internet, containing an application
pub(crate) struct TarXz {
  pub(crate) data: Vec<u8>,
}

impl Archive for TarXz {
  fn extract_all(&self, target_dir: &Path, log: Log) -> Result<()> {
    log(Event::ArchiveExtractBegin { archive_type: "tar.xz" });
    let decompressor = XzDecoder::new(Cursor::new(&self.data));
    let mut archive = tar::Archive::new(decompressor);
    match archive.unpack(target_dir) {
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
