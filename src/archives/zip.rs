use super::Archive;
use crate::filesystem::strip_filepath;
use crate::output::Output;
use crate::yard::Executable;
use crate::{filesystem, Result};
use std::path::Path;
use std::{fs, io};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// a .zip file downloaded from the internet, containing an application
pub struct Zip {}

impl Archive for Zip {
    fn can_extract(&self, filename: &str) -> bool {
        filesystem::has_extension(filename, ".zip")
    }

    fn extract_file(&self, data: Vec<u8>, filepath_in_archive: &str, filepath_on_disk: &Path, output: &dyn Output) -> Result<Executable> {
        print_header(output);
        let mut zip_archive = zip::ZipArchive::new(io::Cursor::new(&data)).expect("cannot read zip data");
        if let Some(parent_dir) = filepath_on_disk.parent() {
            if !parent_dir.exists() {
                std::fs::create_dir_all(parent_dir).unwrap();
            }
        }
        for i in 0..zip_archive.len() {
            let file_in_zip = zip_archive.by_index(i).unwrap();
            super::log_archive_file(CATEGORY, file_in_zip.name(), output);
        }
        let mut file_in_zip = zip_archive.by_name(filepath_in_archive).expect("file not found in archive");
        let mut file_on_disk = fs::File::create(filepath_on_disk).unwrap();
        io::copy(&mut file_in_zip, &mut file_on_disk).unwrap();
        #[cfg(unix)]
        file_on_disk.set_permissions(fs::Permissions::from_mode(0o744)).unwrap();
        Ok(Executable(filepath_on_disk.to_path_buf()))
    }

    fn extract_all(&self, data: Vec<u8>, target_dir: &Path, strip_prefix: &str, executable_path_in_archive: &str, output: &dyn Output) -> Result<Executable> {
        print_header(output);
        let mut zip_archive = zip::ZipArchive::new(io::Cursor::new(&data)).expect("cannot read zip data");
        if !target_dir.exists() {
            std::fs::create_dir_all(target_dir).unwrap();
        }
        let mut executable: Option<Executable> = None;
        for i in 0..zip_archive.len() {
            let mut file_in_zip = zip_archive.by_index(i).unwrap();
            let filepath_in_zip = file_in_zip.name();
            super::log_archive_file(CATEGORY, filepath_in_zip, output);
            let filepath_stripped = strip_filepath(filepath_in_zip, strip_prefix);
            let filepath_on_disk = target_dir.join(filepath_stripped);
            let is_executable = filepath_stripped == executable_path_in_archive;
            let mut file_on_disk = fs::File::create(&filepath_on_disk).unwrap();
            io::copy(&mut file_in_zip, &mut file_on_disk).unwrap();
            #[cfg(unix)]
            file_on_disk.set_permissions(fs::Permissions::from_mode(0o744)).unwrap();
            if is_executable {
                executable = Some(Executable(filepath_on_disk));
            }
        }
        executable.ok_or_else(|| panic!("file {executable_path_in_archive} not found in archive"))
    }
}

fn print_header(output: &dyn Output) {
    super::print_header(CATEGORY, "zip", output);
}

const CATEGORY: &str = "extract/zip";
