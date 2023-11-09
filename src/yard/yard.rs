use super::RunnableApp;
use crate::cli::RequestedApp;
use std::path::PathBuf;

pub struct Yard {
    pub root: PathBuf,
}

impl Yard {
    pub fn load(&self, app: &RequestedApp, executable: &str) -> Option<RunnableApp> {
        let file_path = self.file_path(app, executable);
        if file_path.exists() {
            Some(RunnableApp {
                executable: file_path,
            })
        } else {
            None
        }
    }

    pub fn file_path(&self, app: &RequestedApp, file: &str) -> PathBuf {
        self.folder_for(&app).join(file)
    }

    pub fn folder_for(&self, app: &RequestedApp) -> PathBuf {
        self.root.join("apps").join(&app.name).join(&app.version)
    }

    #[cfg(test)]
    fn save(&self, app: &RequestedApp, file_name: &str, file_content: &[u8]) {
        use std::fs;
        use std::io::Write;

        fs::create_dir_all(self.folder_for(app)).unwrap();
        let mut file = fs::File::create(self.file_path(&app, file_name)).unwrap();
        file.write_all(file_content).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::RequestedApp;
    use crate::yard::Yard;
    use big_s::S;
    use std::path::PathBuf;

    #[test]
    fn file_path() {
        let yard = Yard {
            root: PathBuf::from("/root"),
        };
        let app = RequestedApp {
            name: S("shellcheck"),
            version: S("0.9.0"),
        };
        let have = yard.file_path(&app, "shellcheck.exe");
        let want = PathBuf::from("/root/apps/shellcheck/0.9.0/shellcheck.exe");
        assert_eq!(have, want);
    }

    #[test]
    fn folder_for() {
        let yard = Yard {
            root: PathBuf::from("/root"),
        };
        let app = RequestedApp {
            name: S("shellcheck"),
            version: S("0.9.0"),
        };
        let have = yard.folder_for(&app);
        let want = PathBuf::from("/root/apps/shellcheck/0.9.0");
        assert_eq!(have, want);
    }

    mod load {
        use crate::cli::RequestedApp;
        use crate::yard::{create, Yard};
        use big_s::S;
        use std::path::PathBuf;

        #[test]
        fn app_in_installed() {
            let tempdir = tempfile::tempdir().unwrap();
            let yard = create(tempdir.path()).unwrap();
            let requested_app = RequestedApp {
                name: S("shellcheck"),
                version: S("0.9.0"),
            };
            let executable = "executable";
            yard.save(&requested_app, executable, b"content");
            let Some(runnable_app) = yard.load(&requested_app, executable) else {
                panic!();
            };
            assert!(
                runnable_app
                    .executable
                    .to_string_lossy()
                    .ends_with("/apps/shellcheck/0.9.0/executable"),
                "{}",
                runnable_app.executable.to_string_lossy()
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
            let loaded = yard.load(&requested_app, "executable");
            assert!(loaded.is_none());
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
            yard.save(&installed_app, executable, b"content");
            let requested_app = RequestedApp {
                name: S("shellcheck"),
                version: S("0.9.0"),
            };
            let loaded = yard.load(&requested_app, "executable");
            assert!(loaded.is_none());
        }
    }
}
