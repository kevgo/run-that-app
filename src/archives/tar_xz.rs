use super::Archive;
use crate::yard::Executable;
use crate::Output;
use crate::{filesystem, Result};
use std::io::Cursor;
use std::path::Path;
use xz2::read::XzDecoder;

/// a .tar.xz file downloaded from the internet, containing an application
pub struct TarXz {}

impl Archive for TarXz {
    fn can_extract(&self, filename: &str) -> bool {
        filesystem::has_extension(filename, ".tar.xz")
    }

    fn extract(&self, data: Vec<u8>, filepath_in_archive: &str, filepath_on_disk: &Path, output: &dyn Output) -> Result<Executable> {
        output.print("extracting ... ");
        output.log(CATEGORY, "archive type: tar.xz");
        let decompressor = XzDecoder::new(Cursor::new(data));
        let mut archive = tar::Archive::new(decompressor);
        let mut found_file = false;
        for file in archive.entries().unwrap() {
            let mut file = file.unwrap();
            let filepath = file.path().unwrap();
            let filepath = filepath.to_string_lossy();
            output.log(CATEGORY, &format!("- {filepath}"));
            if filepath == filepath_in_archive {
                found_file = true;
                file.unpack(filepath_on_disk).unwrap();
            }
        }
        assert!(found_file, "file {filepath_in_archive} not found in archive");
        filesystem::make_file_executable(filepath_on_disk)?;
        Ok(Executable(filepath_on_disk.to_path_buf()))
    }
}

const CATEGORY: &str = "extract/tar.xz";
