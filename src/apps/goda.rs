use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::Log;
use const_format::formatcp;

pub struct Goda {}

const ORG: &str = "loov";
const REPO: &str = "goda";

impl App for Goda {
  fn name(&self) -> AppName {
    AppName::from("goda")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("help", log)?;
    if !output.contains("Print dependency graph") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // as of 0.5.7 goda has no way to determine the version of the installed executable
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }

  fn install_methods(&self) -> Vec<install::Method> {
    vec![Method::CompileGoSource(self)]
  }
}

impl install::CompileGoSource for Goda {
  fn import_path(&self, version: &Version) -> String {
    format!("github.com/{ORG}/{REPO}@v{version}")
  }
}
