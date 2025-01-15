use big_s::S;

use super::nodejs::NodeJS;
use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::installation::run_other_executable::CallSignature;
use crate::installation::{self, Method};
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{applications, Log};
use std::path;

pub struct Npm {}

impl App for Npm {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("npm")
  }

  fn homepage(&self) -> &'static str {
    "https://www.npmjs.com"
  }

  fn install_methods(&self) -> Vec<installation::Method> {
    vec![Method::ExecutableInAnotherApp(self)]
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    self.app_to_install().latest_installable_version(log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    self.app_to_install().installable_versions(amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output_args(&["help", "npm"], log)?;
    if !output.contains("javascript package manager") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // Npm is versioned together with NodeJS. The actual version of npm is therefore not relevant here.
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

impl installation::RunOtherExecutable for Npm {
  fn app_to_install(&self) -> Box<dyn App> {
    Box::new(NodeJS {})
  }

  fn call_signature(&self, executable: Executable) -> installation::run_other_executable::CallSignature {
    CallSignature {
      executable,
      args: vec![S("node_modules/npm/bin/npm-cli.js")],
    }
  }
}
