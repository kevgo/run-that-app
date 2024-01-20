use super::nodejs::NodeJS;
use super::App;
use crate::platform::{Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};
use std::path::Path;

pub struct Npx {}

impl App for Npx {
    fn name(&self) -> &'static str {
        "npx"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "bin/npx",
            Os::Windows => "bin\\npx.exe",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://www.npmjs.com"
    }

    fn install(&self, version: &str, platform: Platform, folder: &Path, output: &dyn Output) -> Result<Option<Executable>> {
        let nodejs = NodeJS {};
        nodejs.install(version, platform, folder, output)?;
        let executable_path = folder.join(self.executable_filename(platform));
        Ok(Some(Executable(executable_path)))
    }

    fn latest_version(&self, output: &dyn Output) -> Result<String> {
        (NodeJS {}).latest_version(output)
    }

    fn load(&self, version: &str, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app((NodeJS {}).name(), version, self.executable_filename(platform))
    }

    fn versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<String>> {
        (NodeJS {}).versions(amount, output)
    }
}
