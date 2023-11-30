use super::Executable;
use crate::cli::RequestedApp;
use std::path::PathBuf;

pub struct Yard {
  pub root: PathBuf,
}

impl Yard {
    /// provides the path to the executable of the given application
    pub fn load_app(&self, app: &RequestedApp, executable_filename: &str) -> LoadAppOutcome {
        let file_path = self.app_file_path(&app.name, &app.version, executable_filename);
        if file_path.exists() {
            return LoadAppOutcome::Loaded(Executable(file_path));
        }
        if self.is_not_installable(app) {
            return LoadAppOutcome::NotInstallable;
        }
        LoadAppOutcome::NotInstalled
    }
  }

  /// provides the path to the given file that is part of the given application
  pub fn app_file_path(&self, app_name: &str, app_version: &str, file: &str) -> PathBuf {
    self.app_folder(app_name, app_version).join(file)
  }

  /// provides the path to the folder containing the given application
  pub fn app_folder(&self, app_name: &str, app_version: &str) -> PathBuf {
    self.root.join("apps").join(app_name).join(app_version)
  }

    fn is_not_installable(&self, app: &RequestedApp) -> bool {
        self.not_installable_path(&app.name, &app.version).exists()
    }

    /// provides the path to the given file that is part of the given application
    pub fn not_installable_path(&self, app_name: &str, app_version: &str) -> PathBuf {
        self.app_folder(app_name, app_version)
            .join("not_installable")
    }

    /// stores the given application consisting of the given executable file
    #[cfg(test)]
    fn save_app_file(&self, app: &RequestedApp, file_name: &str, file_content: &[u8]) {
        use std::fs;
        use std::io::Write;
        fs::create_dir_all(self.app_folder(&app.name, &app.version)).unwrap();
        let mut file =
            fs::File::create(self.app_file_path(&app.name, &app.version, file_name)).unwrap();
        file.write_all(file_content).unwrap();
    }
}

#[derive(Debug, PartialEq)]
pub enum LoadAppOutcome {
    /// the requested app was loaded from the yard, here is the executable to call
    Loaded(Executable),
    /// the yard doesn't contain this app
    NotInstalled,
    /// a previous run of run-that-app determined that the app cannot be downloaded nor installed for the current platform
    NotInstallable,
}

#[cfg(test)]
mod tests {
  use crate::yard::Yard;
  use std::path::PathBuf;

  #[test]
  fn app_file_path() {
    let yard = Yard { root: PathBuf::from("/root") };
    let have = yard.app_file_path("shellcheck", "0.9.0", "shellcheck.exe");
    let want = PathBuf::from("/root/apps/shellcheck/0.9.0/shellcheck.exe");
    assert_eq!(have, want);
  }

  #[test]
  fn app_folder() {
    let yard = Yard { root: PathBuf::from("/root") };
    let have = yard.app_folder("shellcheck", "0.9.0");
    let want = PathBuf::from("/root/apps/shellcheck/0.9.0");
    assert_eq!(have, want);
  }

  mod load_app {
    use crate::cli::RequestedApp;
    use crate::yard::{create, Executable, Yard};
    use big_s::S;
    use std::path::PathBuf;

    #[test]
    fn app_is_installed() {
      let tempdir = tempfile::tempdir().unwrap();
      let yard = create(tempdir.path()).unwrap();
      let requested_app = RequestedApp {
        name: S("shellcheck"),
        version: S("0.9.0"),
      };
      let executable = "executable";
      yard.save_app_file(&requested_app, executable, b"content");
      let Some(Executable(executable_path)) = yard.load_app(&requested_app, executable) else {
        panic!();
      };
      #[cfg(unix)]
      assert!(
        executable_path.to_string_lossy().ends_with("/apps/shellcheck/0.9.0/executable"),
        "{}",
        executable_path.to_string_lossy()
      );
      #[cfg(windows)]
      assert!(
        executable_path.to_string_lossy().ends_with("\\apps\\shellcheck\\0.9.0\\executable"),
        "{}",
        executable_path.to_string_lossy()
      );
    }

    #[test]
    fn app_is_not_installed() {
      let yard = Yard { root: PathBuf::from("/root") };
      let requested_app = RequestedApp {
        name: S("shellcheck"),
        version: S("0.9.0"),
      };
      let loaded = yard.load_app(&requested_app, "executable");
      assert!(loaded.is_none());
    }

    mod load_app {
        use crate::cli::RequestedApp;
        use crate::yard::{create, Executable, LoadAppOutcome, Yard};
        use big_s::S;
        use std::path::PathBuf;

        #[test]
        fn app_is_installed() {
            let tempdir = tempfile::tempdir().unwrap();
            let yard = create(tempdir.path()).unwrap();
            let requested_app = RequestedApp {
                name: S("shellcheck"),
                version: S("0.9.0"),
            };
            let executable = "executable";
            yard.save_app_file(&requested_app, executable, b"content");
            let LoadAppOutcome::Loaded(Executable(executable_path)) =
                yard.load_app(&requested_app, executable)
            else {
                panic!();
            };
            #[cfg(unix)]
            assert!(
                executable_path
                    .to_string_lossy()
                    .ends_with("/apps/shellcheck/0.9.0/executable"),
                "{}",
                executable_path.to_string_lossy()
            );
            #[cfg(windows)]
            assert!(
                executable_path
                    .to_string_lossy()
                    .ends_with("\\apps\\shellcheck\\0.9.0\\executable"),
                "{}",
                executable_path.to_string_lossy()
            );
        }

        #[test]
        fn app_is_not_installed() {
            let yard = Yard {
                root: PathBuf::from("/root"),
            };
            let requested_app = RequestedApp {
                name: S("shellcheck"),
                version: S("0.9.0"),
            };
            let loaded = yard.load_app(&requested_app, "executable");
            assert_eq!(loaded, LoadAppOutcome::NotInstalled);
        }

        #[test]
        fn app_is_installed_but_wrong_version() {
            let tempdir = tempfile::tempdir().unwrap();
            let yard = create(tempdir.path()).unwrap();
            let installed_app = RequestedApp {
                name: S("shellcheck"),
                version: S("0.1.0"),
            };
            let executable = "executable";
            yard.save_app_file(&installed_app, executable, b"content");
            let requested_app = RequestedApp {
                name: S("shellcheck"),
                version: S("0.9.0"),
            };
            let loaded = yard.load_app(&requested_app, "executable");
            assert_eq!(loaded, LoadAppOutcome::NotInstalled);
        }
    }

    #[test]
    fn not_installable_path() {
        let yard = Yard {
            root: PathBuf::from("/root"),
        };
        let have = yard.not_installable_path("shellcheck", "0.9.0");
        let want = PathBuf::from("/root/apps/shellcheck/0.9.0/not_installable");
        assert_eq!(have, want);
    }
}
