use crate::cli::AppVersion;
use crate::error::UserError;
use crate::subshell::Executable;
use crate::Result;
use std::fs::{self, File};
use std::path::PathBuf;

pub struct Yard {
    pub root: PathBuf,
}

/// stores executables of and metadata about applications
impl Yard {
    /// provides the path to the folder containing the given application
    pub fn app_folder(&self, app_name: &str, app_version: &str) -> PathBuf {
        self.root.join("apps").join(app_name).join(app_version)
    }

    pub fn is_not_installable(&self, app: &AppVersion) -> bool {
        self.not_installable_path(&app.name, &app.version).exists()
    }

    /// provides the path to the executable of the given application
    pub fn load_app(&self, name: &str, version: &str, executable_filename: &str) -> Option<Executable> {
        let file_path = self.app_folder(name, version).join(executable_filename);
        if file_path.exists() {
            Some(Executable(file_path))
        } else {
            None
        }
    }

    pub fn mark_not_installable(&self, app: &AppVersion) -> Result<()> {
        let app_folder = self.app_folder(&app.name, &app.version);
        fs::create_dir_all(&app_folder).map_err(|err| UserError::YardAccessDenied {
            msg: err.to_string(),
            path: app_folder,
        })?;
        let path = self.not_installable_path(&app.name, &app.version);
        match File::create(&path) {
            Ok(_) => Ok(()),
            Err(err) => Err(UserError::YardAccessDenied { msg: err.to_string(), path }),
        }
    }

    /// provides the path to the given file that is part of the given application
    fn not_installable_path(&self, app_name: &str, app_version: &str) -> PathBuf {
        self.app_folder(app_name, app_version).join("not_installable")
    }

    /// stores the given application consisting of the given executable file
    #[cfg(test)]
    fn save_app_file(&self, name: &str, version: &str, file_name: &str, file_content: &[u8]) {
        use std::io::Write;
        fs::create_dir_all(self.app_folder(name, version)).unwrap();
        let mut file = fs::File::create(self.app_folder(name, version).join(file_name)).unwrap();
        file.write_all(file_content).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::yard::Yard;
    use std::path::PathBuf;

    #[test]
    fn app_file_path() {
        let yard = Yard { root: PathBuf::from("/root") };
        let have = yard.app_folder("shellcheck", "0.9.0").join("shellcheck.exe");
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

    mod is_not_installable {
        use crate::cli::AppVersion;
        use crate::yard::create;
        use crate::yard::Yard;
        use big_s::S;
        use std::path::PathBuf;

        #[test]
        fn is_marked() {
            let tempdir = tempfile::tempdir().unwrap();
            let yard = create(tempdir.path()).unwrap();
            let app_version = AppVersion {
                name: S("shellcheck"),
                version: S("0.9.0"),
            };
            yard.mark_not_installable(&app_version).unwrap();
            let have = yard.is_not_installable(&app_version);
            assert!(have);
        }

        #[test]
        fn is_not_marked() {
            let yard = Yard { root: PathBuf::from("/root") };
            let app_version = AppVersion {
                name: S("shellcheck"),
                version: S("0.9.0"),
            };
            let have = yard.is_not_installable(&app_version);
            assert!(!have);
        }
    }

    mod load_app {
        use crate::cli::AppVersion;
        use crate::subshell::Executable;
        use crate::yard::{create, Yard};
        use big_s::S;
        use std::path::PathBuf;

        #[test]
        fn app_is_installed() {
            let tempdir = tempfile::tempdir().unwrap();
            let yard = create(tempdir.path()).unwrap();
            let executable = "executable";
            yard.save_app_file("shellcheck", "0.9.0", executable, b"content");
            let Some(Executable(executable_path)) = yard.load_app("shellcheck", "0.9.0", executable) else {
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
            let app_version = AppVersion {
                name: S("shellcheck"),
                version: S("0.9.0"),
            };
            let loaded = yard.load_app(&app_version.name, &app_version.version, "executable");
            assert!(loaded.is_none());
        }

        #[test]
        fn app_is_installed_but_wrong_version() {
            let tempdir = tempfile::tempdir().unwrap();
            let yard = create(tempdir.path()).unwrap();
            let executable = "executable";
            yard.save_app_file("shellcheck", "0.1.0", executable, b"content");
            let loaded = yard.load_app("shellcheck", "0.9.0", "executable");
            assert!(loaded.is_none());
        }
    }

    #[test]
    fn not_installable_path() {
        let yard = Yard { root: PathBuf::from("/root") };
        let have = yard.not_installable_path("shellcheck", "0.9.0");
        let want = PathBuf::from("/root/apps/shellcheck/0.9.0/not_installable");
        assert_eq!(have, want);
    }
}
