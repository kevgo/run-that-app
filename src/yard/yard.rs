use super::RunnableApp;
use crate::ui::RequestedApp;
use std::path::PathBuf;

pub struct Yard {
    root: PathBuf,
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
}

#[cfg(test)]
mod tests {
    use crate::ui::RequestedApp;
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
        use crate::ui::RequestedApp;
        use crate::yard::Yard;
        use big_s::S;
        use std::fs;
        use std::io::Write;
        use std::path::PathBuf;

        #[test]
        fn app_in_installed() {
            let tempdir = tempfile::tempdir().unwrap();
            let yard = Yard {
                root: tempdir.path().to_path_buf(),
            };
            let requested_app = RequestedApp {
                name: S("shellcheck"),
                version: S("0.9.0"),
            };
            fs::create_dir_all(yard.folder_for(&requested_app)).unwrap();
            let executable_name = "executable";
            let mut file =
                fs::File::create(yard.file_path(&requested_app, &executable_name)).unwrap();
            file.write_all(b"content").unwrap();

            let Some(runnable_app) = yard.load(&requested_app, &executable_name) else {
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
            // TODO
        }
    }
}
