use colored::Colorize;

use crate::output::Output;
use crate::yard::Executable;
use crate::Result;
use std::fs;
use std::path::PathBuf;

/// saves the given uncompressed data as the given file
pub fn save_buffer(
    data: Vec<u8>,
    path_on_disk: PathBuf,
    output: &dyn Output,
) -> Result<Executable> {
    output.print(&format!(
        "saving as {} ... ",
        path_on_disk.to_string_lossy().cyan()
    ));
    fs::write(&path_on_disk, data).expect("cannot save file");
    super::make_file_executable(&path_on_disk)?;
    output.println(&format!("{}", "ok".green()));
    Ok(Executable(path_on_disk))
}
