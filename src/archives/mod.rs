mod tar_gz;
mod tar_xz;
mod zip;

use crate::error::UserError;
use crate::install::ArtifactType;
use crate::output::Output;
use crate::yard::Executable;
use crate::{filesystem, Result};
use std::path::{Path, PathBuf};

/// An archive is a compressed file containing an application.
pub trait Archive {
    /// indicates whether this archive implementation can extract the file with the given name
    fn can_extract(&self, filename: &str) -> bool;

    /// extracts the given file from the given archive file content to the given location on disk
    fn extract_file(&self, data: Vec<u8>, filepath_in_archive: &str, folder_on_disk: &Path, output: &dyn Output) -> Result<Executable>;
}

pub fn extract_file(data: Vec<u8>, path_in_archive: &str, filepath_on_disk: &Path, output: &dyn Output) -> Result<Executable> {
    let extractor = archive
}

/// extracts the given file in the given artifact to the given location on disk
pub fn extract(artifact: Artifact, artifact_type: &ArtifactType, folder_on_disk: &Path, output: &dyn Output) -> Result<Executable> {
    match artifact_type {
        ArtifactType::PackagedExecutable { file_to_extract } => {
            for archive in all_archives() {
                if archive.can_extract(&artifact.filename) {
                    let filepath = PathBuf::from(file_to_extract);
                    let filename = filepath.file_name().unwrap();
                    let file_path_on_disk = folder_on_disk.join(filename);
                    return archive.extract_file(artifact.data, file_to_extract, &file_path_on_disk, output);
                }
            }
            Err(UserError::UnknownArchive(artifact.filename))
        }
        ArtifactType::Executable { filename } => {
            let file_path_on_disk = folder_on_disk.join(filename);
            filesystem::save_buffer(artifact.data, &file_path_on_disk, output)
        }
    }
}

fn all_archives() -> Vec<Box<dyn Archive>> {
    vec![Box::new(tar_gz::TarGz {}), Box::new(tar_xz::TarXz {}), Box::new(zip::Zip {})]
}

/// An artifacts is a file containing an application, downloaded from the internet.
/// An artifact could be an archive containing the application binary (and other files),
/// or the uncompressed application binary itself.
pub struct Artifact {
    pub filename: String,
    pub data: Vec<u8>,
}
