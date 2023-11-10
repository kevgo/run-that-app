//! An archive is a compressed file containing an application.

mod tar_gz;
mod zip;

use crate::download::Artifact;
use crate::output::Output;
use crate::yard::RunnableApp;
use crate::{filesystem, Result};
use std::path::PathBuf;
pub use tar_gz::TarGz;
pub use zip::Zip;

pub trait Archive {
    fn can_extract(&self, filename: &str) -> bool;

    fn extract(
        &self,
        data: Vec<u8>,
        path_in_archive: String,
        path_on_disk: PathBuf,
        output: &dyn Output,
    ) -> Result<RunnableApp>;
}

/// provides an Archive implementation that can extract the given artifact
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
    filesystem::save_buffer(artifact.data, path_on_disk, output)
}

fn all_archives() -> Vec<Box<dyn Archive>> {
    vec![Box::new(TarGz {}), Box::new(Zip {})]
}
