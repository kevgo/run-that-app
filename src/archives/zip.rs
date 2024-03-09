use super::Archive;
use crate::error::UserError;
use crate::output::Output;
use crate::Result;
use std::io;
use std::path::Path;

/// a .zip file downloaded from the internet, containing an application
pub struct Zip {
    pub data: Vec<u8>,
}

impl Archive for Zip {
    fn extract_all(&self, target_dir: &Path, output: &dyn Output) -> Result<()> {
        print_header(output);
        let mut zip_archive = zip::ZipArchive::new(io::Cursor::new(&self.data)).expect("cannot read zip data");
        zip_archive.extract(target_dir).map_err(|err| UserError::ArchiveCannotExtract { reason: err.to_string() })
    }
}

fn print_header(output: &dyn Output) {
    super::print_header(CATEGORY, "zip", output);
}

const CATEGORY: &str = "extract/zip";
