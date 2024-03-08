use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::platform::Platform;
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};
use const_format::formatcp;

pub struct Deadcode {}

impl App for Deadcode {
    fn name(&self) -> AppName {
        AppName::from("deadcode")
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
        yard.load_app(&self.name(), version, &self.executable_filepath(platform))
    }

    fn installable_versions(&self, _amount: usize, _output: &dyn Output) -> Result<Vec<Version>> {
        Ok(vec![Version::from("0.16.1")])
    }

    fn analyze_executable(&self, executable: &Executable) -> AnalyzeResult {
        if !identify(&executable.run_output("-h")) {
            return AnalyzeResult::NotIdentified;
        }
        // as of 0.16.1 deadcode does not display the version of the installed executable
        AnalyzeResult::IdentifiedButUnknownVersion
    }
}

fn identify(output: &str) -> bool {
    output.contains("The deadcode command reports unreachable functions in Go programs")
}
