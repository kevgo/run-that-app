use super::nodejs::NodeJS;
use super::App;
use crate::config::{AppName, Version};
use crate::platform::{Os, Platform};
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};

pub struct Npm {}

impl App for Npm {
    fn name(&self) -> AppName {
        AppName::from("npm")
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "npm",
            Os::Windows => "npm.exe",
        }
    }

    fn executable_filepath(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "bin/npm",
            Os::Windows => "bin\\npm.exe",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://www.npmjs.com"
    }

    fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        let nodejs = NodeJS {};
        nodejs.install(version, platform, yard, output)?;
        let executable_path = yard.app_folder(&nodejs.name(), version).join(self.executable_filepath(platform));
        Ok(Some(Executable(executable_path)))
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        (NodeJS {}).latest_installable_version(output)
    }

    fn load(&self, version: &Version, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(&(NodeJS {}).name(), version, self.executable_filepath(platform))
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        (NodeJS {}).installable_versions(amount, output)
    }

    fn version(&self, path: &Executable) -> Option<Version> {
        todo!()
    }
}
