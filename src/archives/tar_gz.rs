use super::Archive;
use crate::yard::Executable;
use crate::Output;
use crate::{filesystem, Result};
use flate2::read::GzDecoder;
use std::io;
use std::path::Path;

/// a .tar.gz file downloaded from the internet, containing an application
pub struct TarGz {}

impl Archive for TarGz {
    fn can_extract(&self, filename: &str) -> bool {
        filesystem::has_extension(filename, ".tar.gz")
    }

    fn extract(&self, data: Vec<u8>, filepath_in_archive: &str, filepath_on_disk: &Path, output: &dyn Output) -> Result<Executable> {
        output.print("extracting ... ");
        output.log(CATEGORY, "archive type: tar.gz");
        let gz_decoder = GzDecoder::new(io::Cursor::new(&data));
        let mut archive = tar::Archive::new(gz_decoder);
        for file in archive.entries().unwrap() {
            let mut file = file.unwrap();
            let filepath = file.path().unwrap();
            let filepath = filepath.to_string_lossy();
            output.log(CATEGORY, &format!("- {filepath}"));
            if filepath == filepath_in_archive {
                file.unpack(filepath_on_disk).unwrap();
                filesystem::make_file_executable(filepath_on_disk)?;
                return Ok(Executable(filepath_on_disk.to_path_buf()));
            }
        }
        panic!("file {filepath_in_archive} not found in archive");
    }
}

const CATEGORY: &str = "extract/tar.gz";
