use super::Archive;
use crate::error::UserError;
use crate::Output;
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
        print_header(output);
        let decompressor = XzDecoder::new(Cursor::new(&self.data));
        let mut archive = tar::Archive::new(decompressor);
        archive.unpack(target_dir).map_err(|err| UserError::ArchiveCannotExtract { reason: err.to_string() })
    }
}

fn print_header(output: Output) {
    super::print_header(CATEGORY, "tar.xz", output);
}

const CATEGORY: &str = "extract/tar.xz";
