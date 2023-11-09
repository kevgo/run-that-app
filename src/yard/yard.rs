use super::RunnableApp;
use crate::ui::RequestedApp;
use std::path::PathBuf;

pub struct Yard {
    root: PathBuf,
}

impl Yard {
    pub fn load(&self, app: &RequestedApp) -> Option<RunnableApp> {
        todo!()
    }

    pub fn file_path(&self, app: RequestedApp, file: &str) -> PathBuf {
        self.folder_for(&app).join(file)
    }

    pub fn folder_for(&self, app: &RequestedApp) -> PathBuf {
        self.root.join("apps").join(&app.name).join(&app.version)
    }
}

#[cfg(test)]
mod tests {
    mod folder_for {
        use crate::ui::RequestedApp;
        use crate::yard::Yard;
        use big_s::S;
        use std::path::PathBuf;

        #[test]
        fn foo() {
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
    }

    mod load {

        #[test]
        fn app_in_installed() {
            // TODO
        }

        #[test]
        fn app_is_not_installed() {
            // TODO
        }

        #[test]
        fn app_is_installed_but_wrong_version() {
            // TODO
        }
    }
}
