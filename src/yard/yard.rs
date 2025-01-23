use super::app_folder::AppFolder;
use super::root_path;
use crate::applications::AppDefinition;
use crate::configuration::{ApplicationName, Version};
use crate::prelude::*;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

pub struct Yard {
  pub root: PathBuf,
}

/// stores executables of and metadata about applications
impl Yard {
  pub fn app_folder(&self, app_definition: Box<dyn AppDefinition>, app_version: &Version) -> AppFolder {
    AppFolder {
      root: self.app_folder_path(&app_definition.name(), &app_version),
      app_definition,
    }
  }

  pub fn app_folder_path(&self, app_name: &ApplicationName, version: &Version) -> PathBuf {
    self.root.join("apps").join(app_name).join(version)
  }

  pub fn create(containing_folder: &Path) -> Result<Yard> {
    let root = root_path(containing_folder);
    if let Err(err) = fs::create_dir_all(&root) {
      return Err(UserError::CannotCreateFolder {
        folder: root,
        reason: err.to_string(),
      });
    }
    Ok(Yard { root })
  }

  pub fn create_app_folder(&self, app_definition: Box<dyn AppDefinition>, app_version: &Version) -> Result<AppFolder> {
    let folder = self.app_folder_path(&app_definition.name(), app_version);
    fs::create_dir_all(&folder).map_err(|err| UserError::CannotCreateFolder {
      folder: folder.clone(),
      reason: err.to_string(),
    })?;
    Ok(AppFolder { root: folder, app_definition })
  }

  pub fn delete_app_folder(&self, app_name: &ApplicationName) -> Result<()> {
    let folder_path = self.root.join("apps").join(app_name);
    fs::remove_dir_all(&folder_path).map_err(|err| UserError::CannotDeleteFolder {
      folder: folder_path.to_string_lossy().to_string(),
      err: err.to_string(),
    })?;
    Ok(())
  }

  pub fn is_not_installable(&self, app: &ApplicationName, version: &Version) -> bool {
    self.not_installable_path(app, version).exists()
  }

  pub fn load(containing_folder: &Path) -> Result<Option<Yard>> {
    let root_dir = root_path(containing_folder);
    let Ok(metadata) = root_dir.metadata() else {
      return Ok(None);
    };
    if !metadata.is_dir() {
      return Err(UserError::YardRootIsNotFolder { root: root_dir });
    }
    Ok(Some(Yard { root: root_dir }))
  }

  pub fn load_or_create(containing_folder: &Path) -> Result<Yard> {
    match Yard::load(containing_folder)? {
      Some(existing_yard) => Ok(existing_yard),
      None => Yard::create(containing_folder),
    }
  }

  pub fn mark_not_installable(&self, app_name: &ApplicationName, version: &Version) -> Result<()> {
    let path = self.not_installable_path(app_name, version);
    match File::create(&path) {
      Ok(_) => Ok(()),
      Err(err) => Err(UserError::YardAccessDenied { msg: err.to_string(), path }),
    }
  }

  fn not_installable_path(&self, app_name: &ApplicationName, version: &Version) -> PathBuf {
    self.app_folder_path(app_name, version).join("not_installable")
  }
}

#[cfg(test)]
mod tests {
  use crate::applications::ShellCheck;
  use crate::configuration::{ApplicationName, Version};
  use crate::yard::app_folder::AppFolder;
  use crate::yard::Yard;
  use std::path::PathBuf;

  #[test]
  fn app_folder() {
    let yard = Yard { root: PathBuf::from("/root") };
    let shellcheck = Box::new(ShellCheck {});
    let have = yard.app_folder(shellcheck.clone(), &Version::from("0.9.0"));
    let want = AppFolder {
      root: PathBuf::from("/root/apps/shellcheck/0.9.0"),
      app_definition: shellcheck,
    };
    assert_eq!(have, want);
  }

  mod is_not_installable {
    use crate::configuration::{ApplicationName, Version};
    use crate::yard::Yard;
    use std::path::PathBuf;

    #[test]
    fn is_marked() {
      let tempdir = tempfile::tempdir().unwrap();
      let yard = Yard::create(tempdir.path()).unwrap();
      let app_name = ApplicationName::from("shellcheck");
      let version = Version::from("0.9.0");
      yard.mark_not_installable(&app_name, &version).unwrap();
      let have = yard.is_not_installable(&app_name, &version);
      assert!(have);
    }

    #[test]
    fn is_not_marked() {
      let yard = Yard { root: PathBuf::from("/root") };
      let app_name = ApplicationName::from("shellcheck");
      let version = Version::from("0.9.0");
      let have = yard.is_not_installable(&app_name, &version);
      assert!(!have);
    }
  }

  #[test]
  fn not_installable_path() {
    let yard = Yard { root: PathBuf::from("/root") };
    let have = yard.not_installable_path(&ApplicationName::from("shellcheck"), &Version::from("0.9.0"));
    let want = PathBuf::from("/root/apps/shellcheck/0.9.0/not_installable");
    assert_eq!(have, want);
  }
}
