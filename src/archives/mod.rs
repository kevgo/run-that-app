mod tar_gz;
mod tar_xz;
mod zip;

use crate::download::Artifact;
use crate::error::UserError;
use crate::output::Output;
use crate::yard::Executable;
use crate::Result;
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

fn all_archives() -> Vec<Box<dyn Archive>> {
    vec![Box::new(tar_gz::TarGz {}), Box::new(tar_xz::TarXz {}), Box::new(zip::Zip {})]
}

/// provides the archive that can extract the given file path
fn lookup(filepath: &str) -> Option<Box<dyn Archive>> {
    all_archives().into_iter().find(|archive| archive.can_extract(filepath))
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
