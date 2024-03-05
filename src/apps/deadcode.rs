use super::App;
use crate::config::{AppName, Version};
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::platform::{Os, Platform};
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};
use const_format::formatcp;

pub struct Deadcode {}

impl App for Deadcode {
    fn name(&self) -> AppName {
        AppName::from("deadcode")
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "deadcode",
            Os::Windows => "deadcode.exe",
        }
    }

    fn executable_filepath(&self, platform: Platform) -> &'static str {
        self.executable_filename(platform)
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://pkg.go.dev/golang.org/x/tools/cmd/deadcode")
    }

    fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        compile_go(CompileArgs {
            import_path: format!("golang.org/x/tools/cmd/deadcode@v{version}"),
            target_folder: &yard.app_folder(&self.name(), version),
            executable_filepath: self.executable_filepath(platform),
            output,
        })
    }

    fn latest_installable_version(&self, _output: &dyn Output) -> Result<Version> {
        // TODO: remove this file once deadcode is integrated into golangci-lint
        Ok(Version::from("0.16.1"))
    }

    fn load(&self, version: &Version, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(&self.name(), version, self.executable_filepath(platform))
    }

    fn installable_versions(&self, _amount: usize, _output: &dyn Output) -> Result<Vec<Version>> {
        Ok(vec![Version::from("0.16.1")])
    }
}
