use super::root_path;
use crate::applications::{AppDefinition, ApplicationName};
use crate::configuration::Version;
use crate::installation::BinFolder;
use crate::logging::{Event, Log};
use crate::platform::Platform;
use crate::prelude::*;
use crate::executable::{Executable, ExecutableNameUnix};
use std::fs::{self, File};
use std::path::{Path, PathBuf};

pub(crate) struct Yard {
  pub(crate) root: PathBuf,
}

/// stores executables of and metadata about applications
// Named after rail yards, i.e. locations where passenger cars of trains are stored, sorted, and repaired.
impl Yard {
  pub(crate) fn app_folder(&self, app_name: &ApplicationName, app_version: &Version) -> PathBuf {
    self.root.join("apps").join(app_name).join(app_version)
  }

  pub(crate) fn create(containing_folder: &Path) -> Result<Yard> {
    let root = root_path(containing_folder);
    if let Err(err) = fs::create_dir_all(&root) {
      return Err(UserError::CannotCreateFolder {
        folder: root,
        reason: err.to_string(),
      });
    }
    Ok(Yard { root })
  }

  pub(crate) fn create_app_folder(&self, app_name: &ApplicationName, app_version: &Version) -> Result<PathBuf> {
    let folder = self.app_folder(app_name, app_version);
    fs::create_dir_all(&folder).map_err(|err| UserError::CannotCreateFolder {
      folder: folder.clone(),
      reason: err.to_string(),
    })?;
    Ok(folder)
  }

  pub(crate) fn delete_app_folder(&self, app_name: &ApplicationName) -> Result<()> {
    let folder_path = self.root.join("apps").join(app_name);
    fs::remove_dir_all(&folder_path).map_err(|err| UserError::CannotDeleteFolder {
      folder: folder_path.to_string_lossy().to_string(),
      err: err.to_string(),
    })?;
    Ok(())
  }

  pub(crate) fn is_not_installable(&self, app: &ApplicationName, version: &Version) -> bool {
    self.not_installable_path(app, version).exists()
  }

  pub(crate) fn load(containing_folder: &Path) -> Result<Option<Yard>> {
    let root_dir = root_path(containing_folder);
    let Ok(metadata) = root_dir.metadata() else {
      return Ok(None);
    };
    if !metadata.is_dir() {
      return Err(UserError::YardRootIsNotFolder { root: root_dir });
    }
    Ok(Some(Yard { root: root_dir }))
  }

  pub(crate) fn load_or_create(containing_folder: &Path) -> Result<Yard> {
    match Yard::load(containing_folder)? {
      Some(existing_yard) => Ok(existing_yard),
      None => Yard::create(containing_folder),
    }
  }

  pub(crate) fn load_executable(
    &self,
    app_definition: &dyn AppDefinition,
    executable: &ExecutableNameUnix,
    version: &Version,
    platform: Platform,
    log: Log,
  ) -> Option<(Executable, BinFolder)> {
    let run_method = app_definition.run_method(version, platform);
    let app_folder = self.app_folder(&app_definition.app_name(), version);
    for installation_method in run_method.install_methods() {
      let executable_paths = installation_method.executable_paths(&app_folder, &executable.clone().platform_path(platform.os));
      for executable_path in executable_paths {
        log(Event::YardCheckExistingAppBegin { path: &executable_path });
        if executable_path.exists() {
          log(Event::YardCheckExistingAppFound);
          let bin_folder = installation_method.bin_folder();
          return Some((Executable::from(executable_path), bin_folder));
        }
      }
    }
    log(Event::YardCheckExistingAppNotFound);
    None
  }

  pub(crate) fn mark_not_installable(&self, app: &ApplicationName, version: &Version) -> Result<()> {
    self.create_app_folder(app, version)?;
    let path = self.not_installable_path(app, version);
    match File::create(&path) {
      Ok(_) => Ok(()),
      Err(err) => Err(UserError::YardAccessDenied { msg: err.to_string(), path }),
    }
  }

  fn not_installable_path(&self, app_name: &ApplicationName, app_version: &Version) -> PathBuf {
    self.app_folder(app_name, app_version).join("not_installable")
  }
}

#[cfg(test)]
mod tests {
  use crate::applications;
  use crate::configuration::Version;
  use crate::yard::Yard;
  use std::path::PathBuf;

  #[test]
  fn app_file_path() {
    let yard = Yard { root: PathBuf::from("/root") };
    let apps = applications::all();
    let shellcheck = apps.lookup("shellcheck").unwrap();
    let have = yard.app_folder(&shellcheck.app_name(), &Version::from("0.9.0")).join("shellcheck.exe");
    let want = PathBuf::from("/root/apps/shellcheck/0.9.0/shellcheck.exe");
    assert_eq!(have, want);
  }

  #[test]
  fn app_folder() {
    let apps = applications::all();
    let shellcheck = apps.lookup("shellcheck").unwrap();
    let yard = Yard { root: PathBuf::from("/root") };
    let have = yard.app_folder(&shellcheck.app_name(), &Version::from("0.9.0"));
    let want = PathBuf::from("/root/apps/shellcheck/0.9.0");
    assert_eq!(have, want);
  }

  mod is_not_installable {
    use crate::applications;
    use crate::configuration::Version;
    use crate::yard::Yard;
    use std::path::PathBuf;

    #[test]
    fn is_marked() {
      let apps = applications::all();
      let shellcheck = apps.lookup("shellcheck").unwrap();
      let tempdir = tempfile::tempdir().unwrap();
      let yard = Yard::create(tempdir.path()).unwrap();
      let app_name = shellcheck.app_name();
      let version = Version::from("0.9.0");
      yard.mark_not_installable(&app_name, &version).unwrap();
      let have = yard.is_not_installable(&app_name, &version);
      assert!(have);
    }

    #[test]
    fn is_not_marked() {
      let apps = applications::all();
      let shellcheck = apps.lookup("shellcheck").unwrap();
      let yard = Yard { root: PathBuf::from("/root") };
      let app_name = shellcheck.app_name();
      let version = Version::from("0.9.0");
      let have = yard.is_not_installable(&app_name, &version);
      assert!(!have);
    }
  }

  #[test]
  fn not_installable_path() {
    let apps = applications::all();
    let shellcheck = apps.lookup("shellcheck").unwrap();
    let yard = Yard { root: PathBuf::from("/root") };
    let have = yard.not_installable_path(&shellcheck.app_name(), &Version::from("0.9.0"));
    let want = PathBuf::from("/root/apps/shellcheck/0.9.0/not_installable");
    assert_eq!(have, want);
  }
}
