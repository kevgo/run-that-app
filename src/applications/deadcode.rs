use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::installation::{self, Method};
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::Executable;
use crate::Log;

pub struct Deadcode {}

impl App for Deadcode {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("deadcode")
  }

  fn homepage(&self) -> &'static str {
    "https://pkg.go.dev/golang.org/x/tools/cmd/deadcode"
  }

  fn install_methods(&self, version: &Version, _platform: Platform) -> Vec<installation::Method> {
    vec![Method::CompileGoSource {
      import_path: format!("golang.org/x/tools/cmd/deadcode@v{version}"),
    }]
  }

  fn latest_installable_version(&self, _log: Log) -> Result<Version> {
    // TODO: remove this file once deadcode is integrated into golangci-lint
    Ok(Version::from("0.16.1"))
  }

  fn installable_versions(&self, _amount: usize, _log: Log) -> Result<Vec<Version>> {
    Ok(vec![Version::from("0.16.1")])
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("The deadcode command reports unreachable functions in Go programs") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // as of 0.16.1 deadcode does not display the version of the installed executable
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}
