use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::installation::{self, Method};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::Log;

pub struct Govulncheck {}

impl App for Govulncheck {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("govulncheck")
  }

  fn homepage(&self) -> &'static str {
    "https://pkg.go.dev/golang.org/x/vuln/cmd/govulncheck"
  }

  fn install_methods(&self) -> Vec<installation::Method> {
    vec![Method::CompileGoSource(self)]
  }

  fn latest_installable_version(&self, _log: Log) -> Result<Version> {
    // TODO: remove this file once deadcode is integrated into golangci-lint
    Ok(Version::from("1.1.3"))
  }

  fn installable_versions(&self, _amount: usize, _log: Log) -> Result<Vec<Version>> {
    Ok(vec![Version::from("1.1.3")])
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Govulncheck reports known vulnerabilities in dependencies") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // as of 0.16.1 deadcode does not display the version of the installed executable
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

impl installation::CompileGoSource for Govulncheck {
  fn import_path(&self, version: &Version) -> String {
    format!("golang.org/x/vuln/cmd/govulncheck@v{version}")
  }
}
