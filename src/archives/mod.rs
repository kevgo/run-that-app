mod tar_gz;
mod tar_xz;
mod zip;

use crate::subshell::Executable;
use crate::{filesystem, Output, Result};
use std::path::Path;

use self::tar_gz::TarGz;
use self::tar_xz::TarXz;
use self::zip::Zip;

/// An archive is a compressed file containing an application.
pub trait Archive {
    /// extracts all files from the given archive data to the given location on disk
    fn extract_all(&self, target_dir: &Path, strip_prefix: &str, executable_path_in_archive: &str, output: &dyn Output) -> Result<Executable>;

    /// extracts the given file from the given archive data to the given location on disk
    fn extract_file(&self, filepath_in_archive: &str, folder_on_disk: &Path, output: &dyn Output) -> Result<Executable>;
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

fn print_header(category: &str, archive_type: &str, output: &dyn Output) {
    if output.is_active(category) {
        output.print(&format!("extracting {archive_type} ..."));
    } else {
        output.print("extracting ... ");
    }
}

fn log_archive_file(category: &str, filepath: &str, output: &dyn Output) {
    if output.is_active(category) {
        output.println(&format!("- {filepath}"));
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
