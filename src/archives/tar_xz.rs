use super::Archive;
use crate::filesystem::strip_filepath;
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

    fn extract_file(&self, data: Vec<u8>, filepath_in_archive: &str, filepath_on_disk: &Path, output: &dyn Output) -> Result<Executable> {
        print_header(output);
        let decompressor = XzDecoder::new(Cursor::new(data));
        let mut archive = tar::Archive::new(decompressor);
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

    fn extract_all(&self, data: Vec<u8>, target_dir: &Path, strip_prefix: &str, executable_path_in_archive: &str, output: &dyn Output) -> Result<Executable> {
        print_header(output);
        let decompressor = XzDecoder::new(Cursor::new(data));
        let mut archive = tar::Archive::new(decompressor);
        let mut executable: Option<Executable> = None;
        for file in archive.entries().unwrap() {
            let mut file = file.unwrap();
            let filepath = file.path().unwrap();
            let filepath_str = filepath.to_string_lossy();
            if output.is_active(CATEGORY) {
                output.println(&format!("- {filepath_str}"));
            }
            let filepath_stripped = strip_filepath(&filepath_str, strip_prefix);
            if filepath_stripped.is_empty() {
                continue;
            }
            let filepath_on_disk = target_dir.join(filepath_stripped);
            let is_executable = filepath_stripped == executable_path_in_archive;
            file.unpack(&filepath_on_disk).unwrap();
            let _ = filesystem::make_file_executable(&filepath_on_disk);
            if is_executable {
                executable = Some(Executable(filepath_on_disk));
            }
        }
        executable.ok_or_else(|| panic!("file {executable_path_in_archive} not found in archive"))
    }
}

fn print_header(output: &dyn Output) {
    if output.is_active(CATEGORY) {
        output.print("extracting tar.xz ...");
    } else {
        output.print("extracting ... ");
    }
}

const CATEGORY: &str = "extract/tar.xz";
