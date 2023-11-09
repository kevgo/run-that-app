use super::Archive;
use crate::ui::output::Output;
use crate::yard::RunnableApp;
use crate::Result;
use colored::Colorize;
use std::fs;

#[cfg(unix)]
use std::os::unix::prelude::PermissionsExt;
use std::path::PathBuf;

pub struct Uncompressed {
    pub data: Vec<u8>,
}

impl Archive for Uncompressed {
    fn extract(
        &self,
        _path_in_archive: String,
        path_on_disk: PathBuf,
        output: &dyn Output,
    ) -> Result<RunnableApp> {
        output.print(&format!(
            "saving as {} ... ",
            path_on_disk.to_string_lossy().cyan()
        ));
        fs::write(&path_on_disk, &self.data).expect("cannot save file");
        #[cfg(unix)]
        fs::set_permissions(&path_on_disk, fs::Permissions::from_mode(0o744)).unwrap();
        println!("{}", "ok".green());
        Ok(RunnableApp {
            executable: path_on_disk,
        })
    }
}
