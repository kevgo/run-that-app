use super::nodejs::NodeJS;
use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::Executable;
use crate::Log;
use std::path;

pub struct Prettier {}

const ORG: &str = "prettier";
const REPO: &str = "prettier";

impl App for Prettier {
  fn name(&self) -> AppName {
    AppName::from("prettier")
  }

  fn homepage(&self) -> &'static str {
    "https://prettier.io"
  }

  fn install_methods(&self) -> Vec<install::Method> {
    vec![Method::InstallAnotherApp(self)]
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !identify(&output) {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // Npx is versioned together with NodeJS. The actual version of npm is therefore not relevant here.
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

impl install::ViaAnotherApp for Prettier {
  fn app_to_install(&self) -> Box<dyn App> {
    Box::new(NodeJS {})
  }

  fn executable_path_in_other_app_yard(&self, version: &Version, platform: Platform) -> String {
    format!(
      "node-v{version}-{os}-{cpu}{sep}bin{sep}{executable}",
      os = super::nodejs::os_text(platform.os),
      cpu = super::nodejs::cpu_text(platform.cpu),
      sep = path::MAIN_SEPARATOR,
      executable = self.executable_filename(platform)
    )
  }
}

fn identify(output: &str) -> bool {
  output.contains("Run a command from a local or remote npm package")
}
