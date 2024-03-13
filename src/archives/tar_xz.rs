use super::Archive;
use crate::error::UserError;
use crate::output::{Event, Output};
use crate::Result;
use std::io::Cursor;
use std::path::Path;
use xz2::read::XzDecoder;

/// a .tar.xz file downloaded from the internet, containing an application
pub struct TarXz {
    pub data: Vec<u8>,
}

impl Archive for TarXz {
    fn extract_all(&self, target_dir: &Path, output: Output) -> Result<()> {
        output.log(Event::ArchiveExtractBegin { archive_type: "tar.xz" });
        let decompressor = XzDecoder::new(Cursor::new(&self.data));
        let mut archive = tar::Archive::new(decompressor);
        match archive.unpack(target_dir) {
            Ok(()) => {
                output.log(Event::ArchiveExtractSuccess);
                Ok(())
            }
            Err(err) => {
                output.log(Event::ArchiveExtractFailed { err: err.to_string() });
                Err(UserError::ArchiveCannotExtract { reason: err.to_string() })
            }
        }
    }
}
