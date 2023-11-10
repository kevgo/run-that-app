//! An archive is a compressed file containing an application.

mod tar_gz;
mod uncompressed;
mod zip;

use crate::output::Output;
use crate::yard::RunnableApp;
use crate::{filesystem, Result};
use std::path::PathBuf;
pub use tar_gz::TarGz;
pub use uncompressed::Uncompressed;
pub use zip::Zip;

pub trait Archive {
    fn extract(
        &self,
        data: Vec<u8>,
        path_in_archive: String,
        path_on_disk: PathBuf,
        output: &dyn Output,
    ) -> Result<RunnableApp>;
}

/// provides an Archive implementation that can extract the given artifact
pub fn lookup(archive_filename: &str) -> Box<dyn Archive> {
    if filesystem::has_extension(archive_filename, ".tar.gz") {
        return Box::new(TarGz {});
    }
    if filesystem::has_extension(archive_filename, ".zip") {
        return Box::new(Zip {});
    }
    Box::new(Uncompressed {})
}
