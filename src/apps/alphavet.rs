use super::App;
use crate::hosting::github;
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::output::Output;
use crate::platform::{Os, Platform};
use crate::yard::{Executable, Yard};
use crate::Result;

pub struct Alphavet {}

const ORG: &str = "skx";
const REPO: &str = "alphavet";

impl App for Alphavet {
    fn name(&self) -> &'static str {
        "alphavet"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "alphavet.exe",
            Os::Linux | Os::MacOS => "alphavet",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://github.com/skx/alphavet"
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        // the precompiled binaries are crashing on Linux
        compile_go(&CompileArgs {
            import_path: format!("github.com/{ORG}/{REPO}/cmd/alphavet@v{version}"),
            target_folder: yard.app_folder(self.name(), version),
            executable_filename: self.executable_filename(platform),
            output,
        })
    }

    fn versions(&self, amount: u8, output: &dyn Output) -> Result<Vec<String>> {
        github::versions(ORG, REPO, amount, output)
    }
}
