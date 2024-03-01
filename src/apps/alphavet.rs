use super::App;
use crate::config::Version;
use crate::hosting::github_releases;
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::platform::{Os, Platform};
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};
use const_format::formatcp;

pub struct Alphavet {}

const ORG: &str = "skx";
const REPO: &str = "alphavet";

impl App for Alphavet {
    fn name(&self) -> &'static str {
        "alphavet"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "alphavet",
            Os::Windows => "alphavet.exe",
        }
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        // the precompiled binaries are crashing on Linux
        compile_go(CompileArgs {
            import_path: format!("github.com/{ORG}/{REPO}/cmd/alphavet@v{version}"),
            target_folder: &yard.app_folder(self.name(), version),
            executable_filename: self.executable_filename(platform),
            output,
        })
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<String> {
        github_releases::latest(ORG, REPO, output)
    }

    fn load(&self, version: &Version, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(self.name(), version, self.executable_filename(platform))
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<String>> {
        github_releases::versions(ORG, REPO, amount, output)
    }
}
