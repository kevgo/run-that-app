use super::Archive;
use crate::logger::{Event, Log};
use crate::prelude::*;
use flate2::read::GzDecoder;
use std::io;
use std::path::Path;

/// a .tar.gz file downloaded from the internet, containing an application
pub struct TarGz {
    pub data: Vec<u8>,
}

impl Archive for TarGz {
    fn extract_all(&self, target_dir: &Path, log: Log) -> Result<()> {
        log(Event::ArchiveExtractBegin { archive_type: "tar.gz" });
        let gz_decoder = GzDecoder::new(io::Cursor::new(&self.data));
        let mut archive = tar::Archive::new(gz_decoder);
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
