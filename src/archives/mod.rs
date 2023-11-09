//! An archive is a compressed file containing an application.

mod tar_gz;
mod uncompressed;
mod zip;

use crate::download::Artifact;
use crate::ui::output::Output;
use crate::yard::RunnableApp;
use crate::Result;
use std::path::PathBuf;
pub use tar_gz::TarGz;
pub use uncompressed::Uncompressed;
pub use zip::Zip;

pub trait Archive {
    fn extract(
        &self,
        file: String,
        path_on_disk: PathBuf,
        output: &dyn Output,
    ) -> Result<RunnableApp>;
}

/// provides an Archive implementation that can extract the given artifact
pub fn lookup(artifact: Artifact) -> Box<dyn Archive> {
    if artifact.filename.ends_with(".tar.gz") {
        return Box::new(TarGz {
            data: artifact.data,
        });
    }
    if artifact.filename.ends_with(".zip") {
        return Box::new(Zip {
            data: artifact.data,
        });
    }
    Box::new(Uncompressed {
        data: artifact.data,
    })
}
