use super::App;
use crate::hosting::github;
use crate::install::{CompileFromGoSource, InstallationMethod};
use crate::output::Output;
use crate::platform::{Os, Platform};
use crate::yard::Yard;
use crate::Result;

pub struct Alphavet {}

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

    fn installation_methods(&self, version: &str, platform: Platform, yard: &Yard) -> Vec<Box<dyn InstallationMethod>> {
        vec![
            // the precompiled binaries are crashing on Linux
            Box::new(CompileFromGoSource {
                import_path: format!("github.com/skx/alphavet/cmd/alphavet@v{version}"),
                target_folder: yard.app_folder(self.name(), version),
                executable_filename: self.executable_filename(platform),
            }),
        ]
    }

    fn versions(&self, amount: u8, output: &dyn Output) -> Result<Vec<String>> {
        github::versions("skx", "alphavet", amount, output)
    }
}
