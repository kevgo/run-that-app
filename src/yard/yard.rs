use super::Executable;
use crate::cli::RequestedApp;
use crate::error::UserError;
use crate::Result;
use std::fs::File;
use std::path::PathBuf;

pub struct Yard {
    pub root: PathBuf,
}

/// stores data and executables of applications
impl Yard {
    /// provides the path to the given file that is part of the given application
    pub fn app_file_path(&self, app_name: &str, app_version: &str, file: &str) -> PathBuf {
        self.app_folder(app_name, app_version).join(file)
    }

    /// provides the path to the folder containing the given application
    pub fn app_folder(&self, app_name: &str, app_version: &str) -> PathBuf {
        self.root.join("apps").join(app_name).join(app_version)
    }

    pub fn is_not_installable(&self, app: &RequestedApp) -> bool {
        self.not_installable_path(&app.name, &app.version).exists()
    }

    /// provides the path to the executable of the given application
    pub fn load_app(&self, app: &RequestedApp, executable_filename: &str) -> Option<Executable> {
        let file_path = self.app_file_path(&app.name, &app.version, executable_filename);
        if file_path.exists() {
            return Some(Executable(file_path));
        }
        None
    }

    fn mark_not_installable(&self, app: &RequestedApp) -> Result<()> {
        let path = self.not_installable_path(&app.name, &app.version);
        match File::create(path) {
            Ok(_) => Ok(()),
            Err(err) => Err(UserError::YardAccessDenied { msg: err.to_string(), path }),
        }
    }

    /// provides the path to the given file that is part of the given application
    pub fn not_installable_path(&self, app_name: &str, app_version: &str) -> PathBuf {
        self.app_folder(app_name, app_version).join("not_installable")
    }

    /// stores the given application consisting of the given executable file
    #[cfg(test)]
    fn save_app_file(&self, app: &RequestedApp, file_name: &str, file_content: &[u8]) {
        use std::fs;
        use std::io::Write;
        fs::create_dir_all(self.app_folder(&app.name, &app.version)).unwrap();
        let mut file = fs::File::create(self.app_file_path(&app.name, &app.version, file_name)).unwrap();
        file.write_all(file_content).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::RequestedApp;
    use crate::yard::{LoadAppOutcome, Yard};
    use big_s::S;
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

    #[test]
    fn app_is_marked_not_installable() {
        let yard = Yard { root: PathBuf::from("/root") };
        let requested_app = RequestedApp {
            name: S("shellcheck"),
            version: S("0.9.0"),
        };
        yard.mark_not_installable(&requested_app);
        let loaded = yard.load_app(&requested_app, "executable");
        assert_eq!(loaded, LoadAppOutcome::NotInstallable);
    }

    #[test]
    fn not_installable_path() {
        let yard = Yard { root: PathBuf::from("/root") };
        let have = yard.not_installable_path("shellcheck", "0.9.0");
        let want = PathBuf::from("/root/apps/shellcheck/0.9.0/not_installable");
        assert_eq!(have, want);
    }
}
