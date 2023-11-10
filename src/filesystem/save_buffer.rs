use colored::Colorize;

use crate::output::Output;
use crate::yard::RunnableApp;
use crate::Result;
use std::fs;
use std::path::PathBuf;

/// writes the given uncompressed data under the given filename to disk
pub fn save_buffer(
    data: Vec<u8>,
    path_on_disk: PathBuf,
    output: &dyn Output,
) -> Result<RunnableApp> {
    output.print(&format!(
        "saving as {} ... ",
        path_on_disk.to_string_lossy().cyan()
    ));
    fs::write(&path_on_disk, data).expect("cannot save file");
    super::make_file_executable(&path_on_disk)?;
    output.println(&format!("{}", "ok".green()));
    Ok(RunnableApp {
        executable: path_on_disk,
    })
}
