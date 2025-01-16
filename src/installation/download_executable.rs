use super::Outcome;
use crate::applications::App;
use crate::configuration::Version;
use crate::logging::Log;
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{download, filesystem};
use std::path::PathBuf;

/// downloads an uncompressed precompiled binary
pub fn install(app: &dyn App, url: &str, version: &Version, platform: Platform, optional: bool, yard: &Yard, log: Log) -> Result<Outcome> {
  let Some(artifact) = download::artifact(url, &app.name(), optional, log)? else {
    return Ok(Outcome::NotInstalled);
  };
  let filepath_on_disk = yard.create_app_folder(&app.name(), version)?.join(app.executable_filename(platform));
  let executable_path_1 = filesystem::save_executable(artifact.data, &filepath_on_disk, log)?;
  let executable_path_2 = executable_path(app, version, platform, yard);
  if executable_path_1.0 != executable_path_2 {
    return Err(UserError::InternalError {
      desc: format!(
        "different executable paths returned after downloading an executable: {executable_path_1} and {}",
        executable_path_2.to_string_lossy()
      ),
    });
  }
  if !executable_path_2.exists() {
    return Err(UserError::InternalError {
      desc: format!("downloaded application binary not found on disk at {}", executable_path_2.to_string_lossy()),
    });
  }
  Ok(Outcome::Installed {
    executable: Executable(executable_path_2),
  })
}

pub fn executable_path(app: &dyn App, version: &Version, platform: Platform, yard: &Yard) -> PathBuf {
  yard.app_folder(&app.name(), version).join(app.executable_filename(platform))
}
