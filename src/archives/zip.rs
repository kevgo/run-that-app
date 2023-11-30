use super::Archive;
use crate::output::Output;
use crate::yard::Executable;
use crate::{filesystem, Result};
use std::path::Path;
use std::{fs, io};

/// a .zip file downloaded from the internet, containing an application
pub struct Zip {}

impl Archive for Zip {
  fn can_extract(&self, filename: &str) -> bool {
    filesystem::has_extension(filename, ".zip")
  }

  fn extract(&self, data: Vec<u8>, filepath_in_archive: &str, filepath_on_disk: &Path, output: &dyn Output) -> Result<Executable> {
    output.print("extracting ... ");
    output.log(CATEGORY, "archive type: zip");
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
    let mut file_in_zip = zip_archive.by_name(filepath_in_archive).unwrap();
    let mut file_on_disk = fs::File::create(filepath_on_disk).unwrap();
    io::copy(&mut file_in_zip, &mut file_on_disk).unwrap();
    filesystem::make_file_executable(filepath_on_disk)?;
    Ok(Executable(filepath_on_disk.to_path_buf()))
  }
}

const CATEGORY: &str = "extract/zip";
