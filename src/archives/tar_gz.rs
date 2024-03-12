use super::Archive;
use crate::error::UserError;
use crate::Output;
use crate::Result;
use flate2::read::GzDecoder;
use std::io;
use std::path::Path;

/// a .tar.gz file downloaded from the internet, containing an application
pub struct TarGz {
    pub data: Vec<u8>,
}

impl Archive for TarGz {
    fn extract_all(&self, target_dir: &Path, output: Output) -> Result<()> {
        print_header(output);
        let gz_decoder = GzDecoder::new(io::Cursor::new(&self.data));
        let mut archive = tar::Archive::new(gz_decoder);
        archive.unpack(target_dir).map_err(|err| UserError::ArchiveCannotExtract { reason: err.to_string() })
    }
}

fn print_header(output: Output) {
    super::print_header(CATEGORY, "tar.gz", output);
}

const CATEGORY: &str = "extract/tar.gz";
