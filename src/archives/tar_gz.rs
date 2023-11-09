use super::Archive;
use crate::ui::Output;
use crate::yard::RunnableApp;
use crate::Result;
use colored::Colorize;
use flate2::read::GzDecoder;
use std::io;

#[cfg(unix)]
use std::os::unix::prelude::PermissionsExt;
use std::path::PathBuf;

/// a .tar.gz file downloaded from the internet, containing an application
pub struct TarGz {
    pub data: Vec<u8>,
}

impl Archive for TarGz {
    fn extract(
        &self,
        path_in_archive: String,
        path_on_disk: PathBuf,
        output: &dyn Output,
    ) -> Result<RunnableApp> {
        output.print(&format!(
            "extracting {} from tar.gz archive ... ",
            path_on_disk.to_string_lossy().cyan()
        ));
        let tar = GzDecoder::new(io::Cursor::new(&self.data));
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
        #[cfg(unix)]
        std::fs::set_permissions(&path_on_disk, std::fs::Permissions::from_mode(0o744)).unwrap();
        println!("{}", "ok".green());
        Ok(RunnableApp {
            executable: path_on_disk,
        })
    }
}

const CATEGORY: &str = "extract/tar.gz";
