use super::Archive;
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
        if output.is_active(CATEGORY) {
            output.print("extracting zip ...");
        } else {
            output.print("extracting ... ");
        }
        let mut zip_archive = zip::ZipArchive::new(io::Cursor::new(&data)).expect("cannot read zip data");
        if let Some(parent_dir) = filepath_on_disk.parent() {
            if !parent_dir.exists() {
                std::fs::create_dir_all(parent_dir).unwrap();
            }
        }
        for i in 0..zip_archive.len() {
            let file_in_zip = zip_archive.by_index(i).unwrap();
            output.log(CATEGORY, &format!("- {}", file_in_zip.name()));
        }
        let mut file_in_zip = zip_archive.by_name(filepath_in_archive).expect("file not found in archive");
        let mut file_on_disk = fs::File::create(filepath_on_disk).unwrap();
        io::copy(&mut file_in_zip, &mut file_on_disk).unwrap();
        #[cfg(unix)]
        file_on_disk.set_permissions(fs::Permissions::from_mode(0o744)).unwrap();
        Ok(Executable(filepath_on_disk.to_path_buf()))
    }
}

const CATEGORY: &str = "extract/zip";
