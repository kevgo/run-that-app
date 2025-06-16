use super::{AnalyzeResult, AppDefinition};
use crate::Log;
use crate::configuration::Version;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::Platform;
use crate::prelude::*;
use const_format::formatcp;

pub(crate) struct FuncOrder {}

const ORG: &str = "manuelarte";
const REPO: &str = "funcorder";

impl AppDefinition for FuncOrder {
  fn name(&self) -> &'static str {
    "funcorder"
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, _platform: Platform) -> RunMethod {
    RunMethod::ThisApp {
      install_methods: vec![Method::CompileGoSource {
        import_path: format!("github.com/{ORG}/{REPO}@v{version}"),
      }],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("checks the order of functions, methods, and constructors") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }

  fn clone(&self) -> Box<dyn AppDefinition> {
    Box::new(Self {})
  }
}
