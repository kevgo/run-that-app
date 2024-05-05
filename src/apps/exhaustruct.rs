use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::Log;
use const_format::formatcp;

pub struct Exhaustruct {}

const ORG: &str = "GaijinEntertainment";
const REPO: &str = "go-exhaustruct";

impl App for Exhaustruct {
  fn name(&self) -> AppName {
    AppName::from("exhaustruct")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn install_methods(&self) -> Vec<install::Method> {
    vec![Method::CompileGoSource(self)]
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !identify(&output) {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

impl install::CompileGoSource for Exhaustruct {
  fn import_path(&self, version: &Version) -> String {
    format!("github.com/{ORG}/{REPO}/v3/cmd/exhaustruct@v{version}")
  }
}

fn identify(output: &str) -> bool {
  output.contains("golang analyzer that finds structures with uninitialized fields")
}
