mod tar_gz;
mod zip;

use crate::download::Artifact;
use crate::output::Output;
use crate::yard::RunnableApp;
use crate::{filesystem, Result};
use std::path::PathBuf;
pub use tar_gz::TarGz;
pub use zip::Zip;

/// An archive is a compressed file containing an application.
pub trait Archive {
    /// indicates whether this archive implementation can extract the file with the given name
    fn can_extract(&self, filename: &str) -> bool;

    /// extracts the given file from the given archive file content to the given location on disk
    fn extract(
        &self,
        data: Vec<u8>,
        path_in_archive: String,
        path_on_disk: PathBuf,
        output: &dyn Output,
    ) -> Result<RunnableApp>;
}

/// extracts the given file in the given artifact to the given location on disk
pub fn extract(
    artifact: Artifact,
    path_in_archive: String,
    path_on_disk: PathBuf,
    output: &dyn Output,
) -> Result<RunnableApp> {
    for archive in all_archives() {
        if archive.can_extract(&artifact.filename) {
            return archive.extract(artifact.data, path_in_archive, path_on_disk, output);
        }
    }
    // here the file doesn't match any of the known archives --> we assume its the binary itself
    filesystem::save_buffer(artifact.data, path_on_disk, output)
}

fn all_archives() -> Vec<Box<dyn Archive>> {
    vec![Box::new(TarGz {}), Box::new(Zip {})]
}
