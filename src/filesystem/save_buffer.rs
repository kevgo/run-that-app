use crate::logger::{Event, Log};
use crate::subshell::Executable;
use crate::Result;
use std::fs;
use std::path::Path;

/// saves the given file data as an executable file
pub fn save_executable(data: Vec<u8>, path_on_disk: &Path, log: Log) -> Result<Executable> {
    log(Event::ExecutableInstallSaveBegin);
    match fs::write(path_on_disk, data) {
        Ok(()) => log(Event::ExecutableInstallSaveSuccess),
        Err(err) => log(Event::ExecutableInstallSaveFail { err: err.to_string() }),
    }
    super::make_file_executable(path_on_disk)?;
    Ok(Executable(path_on_disk.to_path_buf()))
}
