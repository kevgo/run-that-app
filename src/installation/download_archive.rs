use super::{BinFolder, Outcome};
use crate::applications::AppDefinition;
use crate::configuration::Version;
use crate::logging::Log;
use crate::platform::Platform;
use crate::prelude::*;
use crate::yard::Yard;
use crate::{archives, download};
#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

/// downloads and unpacks the content of an archive file
pub(crate) fn run(
  app_definition: &dyn AppDefinition,
  version: &Version,
  url: &str,
  bin_folders: &BinFolder,
  optional: bool,
  platform: Platform,
  yard: &Yard,
  log: Log,
) -> Result<Outcome> {
  let (app_to_install, executable_name, _args) = app_definition.carrier(version, platform);
  let Some(artifact) = download::artifact(url, &app_to_install.name(), optional, log)? else {
    return Ok(Outcome::NotInstalled);
  };
  let app_folder = yard.create_app_folder(&app_to_install.name(), version)?;
  let Some(archive) = archives::lookup(&artifact.filename, artifact.data) else {
    return Err(UserError::UnknownArchive(artifact.filename));
  };
  // extract the archive
  archive.extract_all(&app_folder, log)?;
  let executable_filename = executable_name.platform_path(platform.os);
  // verify that all executables that should be there exist and are executable
  println!("app folder: {}", app_folder.to_string_lossy());
  println!("bin folders: {}", bin_folders);
  for executable_path in bin_folders.executable_paths(&app_folder, &executable_filename) {
    println!("executable path: {}", executable_path.to_string_lossy());
    make_executable(&executable_path, log);
    // set the executable bit of all executable files that this app provides
    for other_executable in app_definition.additional_executables() {
      let other_executable_filename = other_executable.platform_path(platform.os);
      for other_executable_path in bin_folders.executable_paths(&app_folder, &other_executable_filename) {
        make_executable(&other_executable_path, log);
      }
    }
  }
  Ok(Outcome::Installed)
}

fn make_executable(filepath: &Path, log: Log) {
  #[cfg(unix)]
  let _ = make_executable_unix(filepath, log);
  #[cfg(windows)]
  make_executable_windows();
}

#[cfg(windows)]
fn make_executable_windows() {
  // Windows does not have file permissions --> nothing to do here
}

#[cfg(unix)]
fn make_executable_unix(filepath: &Path, log: Log) -> Result<()> {
  use crate::logging::Event;

  log(Event::MakeExecutable { file: &filepath });
  let Ok(executable_file) = fs::File::open(filepath) else {
    return Err(UserError::ArchiveDoesNotContainExecutable {
      expected: filepath.to_string_lossy().to_string(),
    });
  };
  let metadata = match executable_file.metadata() {
    Ok(metadata) => metadata,
    Err(err) => {
      return Err(UserError::CannotReadFileMetadata { err: err.to_string() });
    }
  };
  let mut permissions = metadata.permissions();
  if permissions.mode() & 0o100 == 0 {
    permissions.set_mode(0o744);
    if let Err(err) = fs::set_permissions(filepath, permissions) {
      return Err(UserError::CannotSetFilePermissions {
        path: filepath.to_string_lossy().to_string(),
        err: err.to_string(),
      });
    }
  }
  Ok(())
}
