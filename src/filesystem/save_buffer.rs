use crate::executables::Executable;
use crate::filesystem;
use crate::logging::{Event, Log};
use std::fs;
use std::path::Path;

/// saves the given file data as an executable file
pub(crate) fn save_executable(data: Vec<u8>, path_on_disk: &Path, log: Log) -> Executable {
  log(Event::ExecutableInstallSaveBegin);
  match fs::write(path_on_disk, data) {
    Ok(()) => log(Event::ExecutableInstallSaveSuccess),
    Err(err) => log(Event::ExecutableInstallSaveFail { err: err.to_string() }),
  }
  filesystem::set_executable_bit(path_on_disk);
  Executable::from(path_on_disk)
}
