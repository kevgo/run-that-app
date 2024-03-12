use crate::output::Output;
use crate::subshell::Executable;
use crate::Result;
use std::fs;
use std::path::Path;

/// saves the given file data as an executable file
pub fn save_executable(data: Vec<u8>, path_on_disk: &Path, output: Output) -> Result<Executable> {
    output.print("saving ... ");
    fs::write(path_on_disk, data).expect("cannot save file");
    super::make_file_executable(path_on_disk)?;
    Ok(Executable(path_on_disk.to_path_buf()))
}
