use colored::Colorize;

use super::Archive;
use crate::ui::Output;
use crate::Result;

/// a .tar.gz file downloaded from the internet, containing an application
pub struct TarGz {
    pub data: Vec<u8>,
}

impl Archive for TarGz {
    fn extract(
        &self,
        files: Vec<String>,
        target: &std::path::Path,
        output: &dyn Output,
    ) -> Result<crate::yard::RunnableApp> {
        print!(
            "extracting {} from tar.gz archive ... ",
            target.to_string_lossy().cyan()
        );
        let _ = io::stdout().flush();
        let tar = GzDecoder::new(io::Cursor::new(archive));
        let mut archive = Archive::new(tar);
        let mut found_file = false;
        for file in archive.entries().unwrap() {
            let mut file = file.unwrap();
            let filepath = file.path().unwrap().to_path_buf();
            let filepath = filepath.to_string_lossy();
            logger.log(CATEGORY, &format!("- {filepath}"));
            if filepath != path_in_archive {
                continue;
            }
            found_file = true;
            file.unpack(path_on_disk).unwrap();
        }
        assert!(found_file, "file {path_in_archive} not found in archive");
        #[cfg(unix)]
        std::fs::set_permissions(path_on_disk, std::fs::Permissions::from_mode(0o744)).unwrap();
        println!("{}", "ok".green());
    }
}
