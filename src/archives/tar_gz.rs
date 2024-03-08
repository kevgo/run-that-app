use super::Archive;
use crate::filesystem::strip_filepath;
use crate::subshell::Executable;
use crate::Output;
use crate::{filesystem, Result};
use flate2::read::GzDecoder;
use std::io;
use std::path::Path;

/// a .tar.gz file downloaded from the internet, containing an application
pub struct TarGz {
    pub data: Vec<u8>,
}

impl Archive for TarGz {
    fn extract_all(&self, target_dir: &Path, strip_prefix: &str, executable_path_in_archive: &str, output: &dyn Output) -> Result<Executable> {
        print_header(output);
        let gz_decoder = GzDecoder::new(io::Cursor::new(&self.data));
        let mut archive = tar::Archive::new(gz_decoder);
        let mut executable: Option<Executable> = None;
        for file in archive.entries().unwrap() {
            let mut file = file.unwrap();
            let filepath = file.path().unwrap();
            let filepath_str = filepath.to_string_lossy();
            super::log_archive_file(CATEGORY, &filepath_str, output);
            let filepath_stripped = strip_filepath(&filepath_str, strip_prefix);
            let filepath_on_disk = target_dir.join(filepath_stripped);
            let is_executable = filepath_stripped == executable_path_in_archive;
            file.unpack(&filepath_on_disk).unwrap();
            filesystem::make_file_executable(&filepath_on_disk)?;
            if is_executable {
                executable = Some(Executable(filepath_on_disk));
            }
        }
        executable.ok_or_else(|| panic!("file {executable_path_in_archive} not found in archive"))
    }
}

fn print_header(output: &dyn Output) {
    super::print_header(CATEGORY, "tar.gz", output);
}

const CATEGORY: &str = "extract/tar.gz";
