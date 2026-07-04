use crate::applications::{AppDefinition, ApplicationName};
use crate::configuration::Version;
use crate::context::RuntimeContext;
use crate::error::{Result, UserError};
use crate::executables::{Executable, ExecutableNameUnix};
use crate::installation::BinFolder;
use crate::logging::{Event, Log};
use crate::yard::root_path;
use fd_lock::RwLock;
use std::fs::{self, File};
use std::path::{Path, PathBuf};

/// The Yard stores application executables and metadata.
/// Named after rail yards, i.e. locations where passenger cars of trains are stored, sorted, and repaired.
pub struct Yard {
  pub root: PathBuf,
}

impl Yard {
  pub fn app_folder(&self, app_name: &ApplicationName, version: &Version) -> PathBuf {
    self.root.join("apps").join(app_version(app_name, version))
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

  pub fn create_app_folder(&self, app_name: &ApplicationName, app_version: &Version) -> Result<PathBuf> {
    let folder = self.app_folder(app_name, app_version);
    fs::create_dir_all(&folder).map_err(|err| UserError::CannotCreateFolder {
      folder: folder.clone(),
      reason: err.to_string(),
    })?;
    Ok(folder)
  }

  pub fn delete_app_folder(&self, app_name: &ApplicationName, version: &Version) -> Result<()> {
    let folder_path = self.root.join("apps").join(app_version(app_name, version));
    if let Err(err) = fs::remove_dir_all(&folder_path)
      && err.kind() != std::io::ErrorKind::NotFound
    {
      return Err(UserError::CannotDeleteFolder {
        folder: folder_path,
        err: err.to_string(),
      });
    }
    Ok(())
  }

  pub fn delete_app_folders(&self, app_name: &ApplicationName, version: Option<&Version>) -> Result<()> {
    if let Some(version) = version {
      self.delete_app_folder(app_name, version)?;
    } else {
      for app_folder in self.find_app_folders(app_name)? {
        fs::remove_dir_all(&app_folder).map_err(|err| UserError::CannotDeleteFolder {
          folder: app_folder,
          err: err.to_string(),
        })?;
      }
    }
    Ok(())
  }

  fn create_lockfile(&self, app_name: &ApplicationName, version: &Version, log: Log) -> Result<(File, PathBuf)> {
    // fast path: try to create the lockfile directly
    let lock_folder = self.lock_folder();
    let lock_path = lock_folder.join(app_version(app_name, version));
    log(Event::FileCreateBegin {
      filename: &lock_path.display(),
    });
    if let Ok(file) = File::create(&lock_path) {
      log(Event::FileCreateSuccess);
      return Ok((file, lock_path));
    }
    // slow path: if the lockfile doesn't exist, create the lock folder and try creating the lockfile again
    self.create_lock_folder(log)?;
    log(Event::FileCreateBegin {
      filename: &lock_path.display(),
    });
    match File::create(&lock_path) {
      Ok(file) => {
        log(Event::FileCreateSuccess);
        Ok((file, lock_path))
      }
      Err(err) => {
        log(Event::FileCreateFail { err: &err });
        Err(UserError::CannotCreateFile {
          filename: lock_path,
          err: err.to_string(),
        })
      }
    }
  }

  /// creates the folder that contains the lockfiles
  fn create_lock_folder(&self, log: Log) -> Result<()> {
    log(Event::FolderCreateBegin {
      name: &self.lock_folder().display(),
    });
    match fs::create_dir_all(self.lock_folder()) {
      Ok(()) => {
        log(Event::FolderCreateSuccess);
        Ok(())
      }
      Err(err) => {
        log(Event::FolderCreateFail { err: &err });
        Err(UserError::CannotCreateFolder {
          folder: self.lock_folder().clone(),
          reason: err.to_string(),
        })
      }
    }
  }

  fn find_app_folders(&self, app_name: &ApplicationName) -> Result<Vec<PathBuf>> {
    let mut result = Vec::new();
    let prefix = format!("{app_name}@");
    let app_folder = self.root.join("apps");
    let entries = fs::read_dir(&app_folder).map_err(|err| UserError::CannotReadFolder {
      folder: app_folder,
      err: err.to_string(),
    })?;
    for entry in entries {
      let Ok(entry) = entry else {
        continue;
      };
      let os_filename = entry.file_name();
      let Some(filename) = os_filename.to_str() else {
        continue;
      };
      if filename.starts_with(&prefix) {
        result.push(entry.path());
      }
    }
    Ok(result)
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

  pub fn load_executable(
    &self,
    app_definition: &dyn AppDefinition,
    executable: &ExecutableNameUnix,
    version: &Version,
    ctx: &RuntimeContext,
  ) -> Option<(Executable, BinFolder)> {
    let run_method = app_definition.run_method(version, ctx.platform);
    let app_folder = self.app_folder(&app_definition.name(), version);
    for installation_method in run_method.install_methods() {
      let executable_paths = installation_method.executable_paths(&app_folder, &executable.clone().platform_path(ctx.platform.os));
      for executable_path in executable_paths {
        (ctx.log)(Event::YardCheckExistingAppBegin { path: &executable_path });
        if executable_path.exists() {
          (ctx.log)(Event::YardCheckExistingAppFound);
          let bin_folder = installation_method.bin_folder();
          return Some((Executable::from(executable_path), bin_folder));
        }
        (ctx.log)(Event::YardCheckExistingAppNotFound);
      }
    }
    (ctx.log)(Event::YardCheckExistingAppNotFound);
    None
  }

  pub fn lock_folder(&self) -> PathBuf {
    self.root.join("locks")
  }

  pub fn mark_not_installable(&self, app: &ApplicationName, version: &Version) -> Result<()> {
    self.create_app_folder(app, version)?;
    let path = self.not_installable_path(app, version);
    match File::create(&path) {
      Ok(_) => Ok(()),
      Err(err) => Err(UserError::YardAccessDenied { msg: err.to_string(), path }),
    }
  }

  fn not_installable_path(&self, app_name: &ApplicationName, app_version: &Version) -> PathBuf {
    self.app_folder(app_name, app_version).join(".run-that-app-not-installable")
  }

  /// runs the given function while holding a lock on the app folder
  pub fn with_lock<T>(&self, app_name: &ApplicationName, version: &Version, ctx: &RuntimeContext, f: impl FnOnce() -> Result<T>) -> Result<T> {
    // acquire the lock
    let (lock_file, lock_path) = self.create_lockfile(app_name, version, ctx.log)?;
    (ctx.log)(Event::LockAcquireBegin { app: app_name });
    let mut lock = RwLock::new(lock_file);
    let guard = match lock.write() {
      Ok(guard) => {
        (ctx.log)(Event::LockAcquireSuccess);
        guard
      }
      Err(err) => {
        (ctx.log)(Event::LockAcquireFail { err: &err });
        return Err(UserError::LockCannotAcquire {
          filename: lock_path,
          err: err.to_string(),
        });
      }
    };

    // run the function
    let result = f();

    // release the lock
    (ctx.log)(Event::LockRelease { app: app_name });
    drop(guard);

    // Note: don't delete the lockfile
    // because that would allow another process to create a new file
    // and acquire a lock on that one.

    result
  }
}

/// provides the filename for the file that locks the installation of the given application at the given version.
pub fn app_version(app_name: &ApplicationName, version: &Version) -> String {
  format!("{app_name}@{version}")
}

#[cfg(test)]
mod tests {
  use crate::applications::{AppDefinition, ShellCheck};
  use crate::configuration::Version;
  use crate::yard::Yard;
  use std::path::PathBuf;

  #[test]
  fn app_folder() {
    let yard = Yard { root: "/root".into() };
    let shellcheck = ShellCheck {};
    let have = yard.app_folder(&shellcheck.name(), &Version::from("0.9.0"));
    let want = PathBuf::from("/root/apps/shellcheck@0.9.0");
    assert_eq!(have, want);
  }

  mod create_lockfile {
    use crate::Version;
    use crate::applications::{AppDefinition, ShellCheck};
    use crate::yard::Yard;
    use std::fs;

    #[test]
    fn lock_folder_exists() {
      let tempdir = tempfile::tempdir().unwrap();
      let yard = Yard::create(tempdir.path()).unwrap();
      let lock_path = yard.lock_folder();
      fs::create_dir_all(&lock_path).unwrap();
      let shellcheck = ShellCheck {};
      let version = Version::from("0.9.0");
      yard.create_lockfile(&shellcheck.name(), &version, crate::logging::normal_log).unwrap();
      let want = tempdir.path().join(".run-that-app").join("locks").join("shellcheck@0.9.0");
      assert!(want.exists());
    }

    #[test]
    fn lock_folder_does_not_exist() {
      let tempdir = tempfile::tempdir().unwrap();
      let yard = Yard::create(tempdir.path()).unwrap();
      let shellcheck = ShellCheck {};
      let version = Version::from("0.9.0");
      yard.create_lockfile(&shellcheck.name(), &version, crate::logging::normal_log).unwrap();
      let want = tempdir.path().join(".run-that-app").join("locks").join("shellcheck@0.9.0");
      assert!(want.exists());
    }
  }

  #[test]
  fn is_not_installable() {
    let yard = Yard { root: "/root".into() };
    let shellcheck = ShellCheck {};
    let version = Version::from("0.9.0");
    let have = yard.is_not_installable(&shellcheck.name(), &version);
    assert!(!have);
  }

  #[test]
  fn mark_not_installable() {
    let tempdir = tempfile::tempdir().unwrap();
    let yard = Yard::create(tempdir.path()).unwrap();
    let shellcheck = ShellCheck {};
    let version = Version::from("0.9.0");
    yard.mark_not_installable(&shellcheck.name(), &version).unwrap();
    let have = yard.is_not_installable(&shellcheck.name(), &version);
    assert!(have);
  }

  #[test]
  fn lock_filename() {
    let shellcheck = ShellCheck {};
    let have = super::app_version(&shellcheck.name(), &Version::from("0.9.0"));
    let want = PathBuf::from("shellcheck@0.9.0");
    assert_eq!(have, want);
  }

  #[test]
  fn not_installable_path() {
    let shellcheck = ShellCheck {};
    let yard = Yard { root: "/root".into() };
    let have = yard.not_installable_path(&shellcheck.name(), &Version::from("0.9.0"));
    let want = PathBuf::from("/root/apps/shellcheck@0.9.0/.run-that-app-not-installable");
    assert_eq!(have, want);
  }
}
