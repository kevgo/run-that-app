use super::Archive;
use crate::error::UserError;
use crate::output::{Event, Output};
use crate::Result;
use std::io;
use std::path::Path;

/// a .zip file downloaded from the internet, containing an application
pub struct Zip {
    pub data: Vec<u8>,
}

impl Archive for Zip {
    fn extract_all(&self, target_dir: &Path, output: Output) -> Result<()> {
        output.log(Event::ArchiveExtractBegin { archive_type: "zip" });
        let mut zip_archive = zip::ZipArchive::new(io::Cursor::new(&self.data)).expect("cannot read zip data");
        match zip_archive.extract(target_dir) {
            Ok(_) => {
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
