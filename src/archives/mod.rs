mod tar_gz;
mod tar_xz;
mod zip;

use self::tar_gz::TarGz;
use self::tar_xz::TarXz;
use self::zip::Zip;
use crate::{filesystem, Output, Result};
use std::path::Path;

/// An archive is a compressed file containing an application.
pub trait Archive {
    /// extracts all files from the given archive data to the given location on disk
    fn extract_all(&self, target_dir: &Path, output: Output) -> Result<()>;
}

/// provides the archive that can extract the given file path
pub fn lookup(filepath: &str, data: Vec<u8>) -> Option<Box<dyn Archive>> {
    match () {
        () if filesystem::has_extension(filepath, ".zip") => Some(Box::new(Zip { data })),
        () if filesystem::has_extension(filepath, ".tar.gz") => Some(Box::new(TarGz { data })),
        () if filesystem::has_extension(filepath, ".tar.xz") => Some(Box::new(TarXz { data })),
        () => None,
    }
}

#[cfg(test)]
mod tests {

    mod lookup {
        use crate::archives::lookup;

        #[test]
        fn known_archive_type() {
            let have = lookup("archive.zip", vec![]);
            assert!(have.is_some());
        }

        #[test]
        fn unknown_archive_type() {
            let have = lookup("archive.zonk", vec![]);
            assert!(have.is_none());
        }
    }
}
