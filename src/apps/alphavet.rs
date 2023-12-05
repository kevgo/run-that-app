use super::App;
use crate::install::{CompileFromGoSource, InstallationMethod};
use crate::platform::{Os, Platform};
use crate::yard::Yard;

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

    fn install(&self, version: &str, platform: Platform, yard: &Yard) -> Result<Option<Executable>> {
        todo!()
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
}
