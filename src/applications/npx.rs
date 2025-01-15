use super::nodejs::NodeJS;
use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::installation::{self, Method};
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::{CallSignature, Executable};
use crate::{applications, Log};
use std::path;

pub struct Npx {}

impl App for Npx {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("npx")
  }

  fn homepage(&self) -> &'static str {
    "https://www.npmjs.com"
  }

  fn install_methods(&self) -> Vec<installation::Method> {
    vec![Method::InstallAnotherApp(self)]
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    (NodeJS {}).latest_installable_version(log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    (NodeJS {}).installable_versions(amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Run a command from a local or remote npm package") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // Npx is versioned together with NodeJS. The actual version of npm is therefore not relevant here.
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

impl installation::PartOfAnotherApp for Npx {
  fn app_to_install(&self) -> Box<dyn App> {
    Box::new(NodeJS {})
  }

  fn call_signature_for_other_app(&self, platform: Platform) -> CallSignature<String> {
    let os = applications::nodejs::os_text(platform.os);
    let cpu = applications::nodejs::cpu_text(platform.cpu);
    let sep = path::MAIN_SEPARATOR;
    let executable = self.executable_filename(platform);
    CallSignature {
      executable: format!("node-v{version}-{os}-{cpu}{sep}bin{sep}{executable}"),
      arguments: todo!(),
    }
  }
}
