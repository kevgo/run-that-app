use super::App;
use crate::hosting::github_releases;
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::platform::{Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};
use const_format::formatcp;

pub struct Goda {}

const ORG: &str = "loov";
const REPO: &str = "goda";

impl App for Goda {
    fn name(&self) -> &'static str {
        "goda"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "goda",
            Os::Windows => "goda.exe",
        }
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        compile_go(CompileArgs {
            import_path: format!("github.com/{ORG}/{REPO}@v{version}"),
            target_folder: yard.app_folder(self.name(), version),
            executable_filename: self.executable_filename(platform),
            output,
        })
    }

    fn latest_version(&self, output: &dyn Output) -> Result<String> {
        github_releases::latest(ORG, REPO, output)
    }

    fn load(&self, version: &str, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(self.name(), version, self.executable_filename(platform))
    }

    fn versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<String>> {
        github_releases::versions(ORG, REPO, amount, output)
    }
}
