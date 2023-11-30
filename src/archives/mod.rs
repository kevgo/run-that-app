mod tar_gz;
mod tar_xz;
mod zip;

use crate::download::Artifact;
use crate::error::UserError;
use crate::install::ArtifactType;
use crate::output::Output;
use crate::yard::Executable;
use crate::{filesystem, Result};
use std::path::Path;

/// An archive is a compressed file containing an application.
pub trait Archive {
  /// indicates whether this archive implementation can extract the file with the given name
  fn can_extract(&self, filename: &str) -> bool;

  /// extracts the given file from the given archive file content to the given location on disk
  fn extract(&self, data: Vec<u8>, filepath_in_archive: &str, filepath_on_disk: &Path, output: &dyn Output) -> Result<Executable>;
}

/// extracts the given file in the given artifact to the given location on disk
pub fn extract(artifact: Artifact, artifact_type: &ArtifactType, filepath_on_disk: &Path, output: &dyn Output) -> Result<Executable> {
  match artifact_type {
    ArtifactType::Archive { file_to_extract } => {
      for archive in all_archives() {
        if archive.can_extract(&artifact.filename) {
          return archive.extract(artifact.data, file_to_extract, filepath_on_disk, output);
        }
      }
      Err(UserError::UnknownArchive(artifact.filename))
    }
    ArtifactType::Executable => filesystem::save_buffer(artifact.data, filepath_on_disk, output),
  }
}

fn all_archives() -> Vec<Box<dyn Archive>> {
  vec![Box::new(tar_gz::TarGz {}), Box::new(tar_xz::TarXz {}), Box::new(zip::Zip {})]
}
