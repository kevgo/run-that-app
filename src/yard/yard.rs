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

    pub fn folder_for(&self, app: &RequestedApp) -> PathBuf {
        PathBuf::new()
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
            let temp_dir = tempfile::tempdir().unwrap();
            let yard = Yard {
                root: temp_dir.path().to_path_buf(),
            };
            let app = RequestedApp {
                name: S("shellcheck"),
                version: S("0.9.0"),
            };
            let have = yard.folder_for(&app);
            let want = PathBuf::from("");
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
