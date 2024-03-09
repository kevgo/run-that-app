use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::install::{self, CompileFromGoSource, Method};
use crate::subshell::Executable;
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

    fn install_methods(&self) -> Vec<install::Method> {
        vec![Method::CompileGoSource(self)]
    }

    fn latest_installable_version(&self, _output: &dyn Output) -> Result<Version> {
        // TODO: remove this file once deadcode is integrated into golangci-lint
        Ok(Version::from("0.16.1"))
    }

    fn installable_versions(&self, _amount: usize, _output: &dyn Output) -> Result<Vec<Version>> {
        Ok(vec![Version::from("0.16.1")])
    }

    fn analyze_executable(&self, executable: &Executable) -> AnalyzeResult {
        if !executable.run_output("-h").contains("The deadcode command reports unreachable functions in Go programs") {
            return AnalyzeResult::NotIdentified;
        }
        // as of 0.16.1 deadcode does not display the version of the installed executable
        AnalyzeResult::IdentifiedButUnknownVersion
    }
}

impl CompileFromGoSource for Deadcode {
    fn import_path(&self, version: &Version) -> String {
        format!("golang.org/x/tools/cmd/deadcode@v{version}")
    }
}
