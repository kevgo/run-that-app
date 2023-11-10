use super::Archive;
use crate::output::Output;
use crate::yard::RunnableApp;
use crate::{filesystem, Result};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

pub struct Uncompressed {}

impl Archive for Uncompressed {
    fn extract(
        &self,
        data: Vec<u8>,
        _path_in_archive: String,
        path_on_disk: PathBuf,
        output: &dyn Output,
    ) -> Result<RunnableApp> {
        output.print(&format!(
            "saving as {} ... ",
            path_on_disk.to_string_lossy().cyan()
        ));
        fs::write(&path_on_disk, data).expect("cannot save file");
        filesystem::make_file_executable(&path_on_disk)?;
        println!("{}", "ok".green());
        Ok(RunnableApp {
            executable: path_on_disk,
        })
    }
}
