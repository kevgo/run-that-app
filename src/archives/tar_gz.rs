use super::Archive;
use crate::yard::RunnableApp;
use crate::Output;
use crate::{filesystem, Result};
use colored::Colorize;
use flate2::read::GzDecoder;
use std::io;
use std::path::PathBuf;

/// a .tar.gz file downloaded from the internet, containing an application
pub struct TarGz {}

impl Archive for TarGz {
    fn can_extract(&self, filename: &str) -> bool {
        filesystem::has_extension(filename, ".tar.gz")
    }

    fn extract(
        &self,
        data: Vec<u8>,
        path_in_archive: String,
        path_on_disk: PathBuf,
        output: &dyn Output,
    ) -> Result<RunnableApp> {
        output.print(&format!(
            "extracting {} from tar.gz archive ... ",
            path_on_disk.to_string_lossy().cyan()
        ));
        let tar = GzDecoder::new(io::Cursor::new(&data));
        let mut archive = tar::Archive::new(tar);
        let mut found_file = false;
        for file in archive.entries().unwrap() {
            let mut file = file.unwrap();
            let filepath = file.path().unwrap().to_path_buf();
            let filepath = filepath.to_string_lossy();
            output.log(CATEGORY, &format!("- {filepath}"));
            if filepath != path_in_archive {
                continue;
            }
            found_file = true;
            file.unpack(&path_on_disk).unwrap();
        }
        assert!(found_file, "file {path_in_archive} not found in archive");
        filesystem::make_file_executable(&path_on_disk)?;
        println!("{}", "ok".green());
        Ok(RunnableApp {
            executable: path_on_disk,
        })
    }
}

const CATEGORY: &str = "extract/tar.gz";
