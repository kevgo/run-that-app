use super::Archive;
use crate::filesystem::strip_filepath;
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

    fn extract_all(&self, data: Vec<u8>, target_dir: &Path, strip_prefix: &str, executable_path_in_archive: &str, output: &dyn Output) -> Result<Executable> {
        if output.is_active(CATEGORY) {
            output.print("extracting tar.gz ...");
        } else {
            output.print("extracting ... ");
        }
        let gz_decoder = GzDecoder::new(io::Cursor::new(&data));
        let mut archive = tar::Archive::new(gz_decoder);
        if output.is_active(CATEGORY) {
            output.println("\nFiles in archive:");
        }
        let mut result: Option<Executable> = None;
        for file in archive.entries().unwrap() {
            let mut file = file.unwrap();
            let filepath = file.path().unwrap();
            if output.is_active(CATEGORY) {
                output.println(&format!("- {}", filepath.to_string_lossy()));
            }
            let filepath_stripped = strip_filepath(&filepath, strip_prefix);
            let filepath_on_disk = target_dir.join(filepath_stripped);
            file.unpack(filepath_on_disk).unwrap();
            filesystem::make_file_executable(&filepath_on_disk)?;
            if filepath_stripped.to_string_lossy() == executable_path_in_archive {
                result = Some(Executable(filepath_on_disk));
            }
        }
        result.ok_or_else(|| panic!("file {executable_path_in_archive} not found in archive"))
    }

    fn extract_file(&self, data: Vec<u8>, filepath_in_archive: &str, file_path_on_disk: &Path, output: &dyn Output) -> Result<Executable> {
        if output.is_active(CATEGORY) {
            output.print("extracting tar.gz ...");
        } else {
            output.print("extracting ... ");
        }
        let gz_decoder = GzDecoder::new(io::Cursor::new(&data));
        let mut archive = tar::Archive::new(gz_decoder);
        if output.is_active(CATEGORY) {
            output.println("\nFiles in archive:");
        }
        for file in archive.entries().unwrap() {
            let mut file = file.unwrap();
            let filepath = file.path().unwrap();
            let filepath = filepath.to_string_lossy();
            if output.is_active(CATEGORY) {
                output.println(&format!("- {filepath}"));
            }
            if filepath == filepath_in_archive {
                file.unpack(file_path_on_disk).unwrap();
                filesystem::make_file_executable(file_path_on_disk)?;
                return Ok(Executable(file_path_on_disk.to_path_buf()));
            }
        }
        panic!("file {filepath_in_archive} not found in archive");
    }
}

const CATEGORY: &str = "extract/tar.gz";
