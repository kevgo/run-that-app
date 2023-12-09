mod tar_gz;
mod tar_xz;
mod zip;

use crate::error::UserError;
use crate::output::Output;
use crate::yard::Executable;
use crate::{filesystem, Result};
use std::path::Path;

/// An archive is a compressed file containing an application.
pub trait Archive {
    /// indicates whether this archive implementation can extract the file with the given name
    fn can_extract(&self, filename: &str) -> bool;

    /// extracts the given file from the given archive file content to the given location on disk
    fn extract_file(&self, data: Vec<u8>, filepath_in_archive: &str, folder_on_disk: &Path, output: &dyn Output) -> Result<Executable>;
}

/// extracts the given file in the given artifact to the given location on disk
pub fn extract_file(artifact: Artifact, path_in_archive: &str, filepath_on_disk: &Path, output: &dyn Output) -> Result<Executable> {
    let Some(archive) = lookup(&artifact.filename) else {
        return Err(UserError::UnknownArchive(artifact.filename));
    };
    archive.extract_file(artifact.data, path_in_archive, filepath_on_disk, output)
}

pub fn store_executable(artifact: Artifact, filepath_on_disk: &Path, output: &dyn Output) -> Result<Executable> {
    filesystem::save_buffer(artifact.data, filepath_on_disk, output)
}

fn all_archives() -> Vec<Box<dyn Archive>> {
    vec![Box::new(tar_gz::TarGz {}), Box::new(tar_xz::TarXz {}), Box::new(zip::Zip {})]
}

fn lookup(extension: &str) -> Option<Box<dyn Archive>> {
    all_archives().into_iter().find(|archive| archive.can_extract(extension))
}

/// An artifacts is a file containing an application, downloaded from the internet.
/// An artifact could be an archive containing the application binary (and other files),
/// or the uncompressed application binary itself.
pub struct Artifact {
    pub filename: String,
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {

    mod lookup {
        use crate::archives::lookup;

        #[test]
        fn known_archive_type() {
            let have = lookup(".zip").unwrap();
            assert!(have.can_extract(".zip"));
        }

        #[test]
        fn unknown_archive_type() {
            let have = lookup(".zonk");
            assert!(have.is_none());
        }
    }
}
