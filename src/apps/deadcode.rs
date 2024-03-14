use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::install::{self, Method};
use crate::subshell::Executable;
use crate::{LogFn, Result};
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

    fn latest_installable_version(&self, _log: LogFn) -> Result<Version> {
        // TODO: remove this file once deadcode is integrated into golangci-lint
        Ok(Version::from("0.16.1"))
    }

    fn installable_versions(&self, _amount: usize, _log: LogFn) -> Result<Vec<Version>> {
        Ok(vec![Version::from("0.16.1")])
    }

    fn analyze_executable(&self, executable: &Executable, log: LogFn) -> Result<AnalyzeResult> {
        let output = executable.run_output("-h", log)?;
        if !output.contains("The deadcode command reports unreachable functions in Go programs") {
            return Ok(AnalyzeResult::NotIdentified { output });
        }
        // as of 0.16.1 deadcode does not display the version of the installed executable
        Ok(AnalyzeResult::IdentifiedButUnknownVersion)
    }
}

impl install::CompileGoSource for Deadcode {
    fn import_path(&self, version: &Version) -> String {
        format!("golang.org/x/tools/cmd/deadcode@v{version}")
    }
}
