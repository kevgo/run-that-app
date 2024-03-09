use super::go::Go;
use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::platform::{Os, Platform};
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};

pub struct Gofmt {}

impl App for Gofmt {
    fn name(&self) -> AppName {
        AppName::from("gofmt")
    }

    fn executable_filepath(&self, platform: Platform) -> String {
        match platform.os {
            Os::Linux | Os::MacOS => "bin/gofmt".into(),
            Os::Windows => "bin\\gofmt.exe".into(),
        }
    }

    fn homepage(&self) -> &'static str {
        "https://go.dev"
    }

    fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        let go = Go {};
        go.install(version, platform, yard, output)?;
        let executable_path = yard.create_app_folder(&go.name(), version)?.join(self.executable_filepath(platform));
        Ok(Some(Executable(executable_path)))
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        (Go {}).latest_installable_version(output)
    }

    fn load(&self, version: &Version, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(&(Go {}).name(), version, &self.executable_filepath(platform))
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        (Go {}).installable_versions(amount, output)
    }

    fn analyze_executable(&self, executable: &Executable) -> AnalyzeResult {
        if !identify(&executable.run_output("-h")) {
            return AnalyzeResult::NotIdentified;
        }
        // TODO: return the version of Go here
        AnalyzeResult::IdentifiedButUnknownVersion
    }
}

fn identify(output: &str) -> bool {
    output.contains("report all errors (not just the first 10 on different lines)")
}
