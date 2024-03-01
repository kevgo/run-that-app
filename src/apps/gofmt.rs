use super::go::Go;
use super::App;
use crate::config::Version;
use crate::platform::{Os, Platform};
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};

pub struct Gofmt {}

impl App for Gofmt {
    fn name(&self) -> &'static str {
        "gofmt"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "gofmt",
            Os::Windows => "gofmt.exe",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://go.dev"
    }

    fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        let go = Go {};
        go.install(version, platform, yard, output)?;
        let executable_path = yard.app_folder(go.name(), version).join(self.executable_filename(platform));
        Ok(Some(Executable(executable_path)))
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        (Go {}).latest_installable_version(output)
    }

    fn load(&self, version: &Version, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app((Go {}).name(), version, &self.executable_path(platform))
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        (Go {}).installable_versions(amount, output)
    }

    fn version(&self, path: &Executable) -> Option<String> {
        todo!()
    }
}

impl Gofmt {
    fn executable_path(&self, platform: Platform) -> String {
        let executable = self.executable_filename(platform);
        match platform.os {
            Os::Windows => format!("bin\\{executable}"),
            Os::Linux | Os::MacOS => format!("bin/{executable}"),
        }
    }
}
