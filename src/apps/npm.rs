use super::nodejs::NodeJS;
use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::install::{self, Method, ViaAnotherApp};
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::{CallSignature, Executable};
use crate::yard::Yard;
use crate::Log;
use big_s::S;
use std::path;

pub struct Npm {}

impl App for Npm {
  fn name(&self) -> AppName {
    AppName::from("npm")
  }

  fn call_signature(&self, platform: Platform, version: Version, yard: &Yard) -> CallSignature {
    let node = NodeJS {};
    let node_call_signature = node.call_signature(platform, version, yard);
    CallSignature {
      executable_name: node_call_signature.executable_name,
      arguments: vec![S("node_modules/npm/bin/npm.js")],
    }
  }

  fn homepage(&self) -> &'static str {
    "https://www.npmjs.com"
  }

  fn install_methods(&self) -> Vec<install::Method> {
    vec![Method::InstallAnotherApp(self)]
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

impl install::ViaAnotherApp for Npm {
  fn app_to_install(&self) -> Box<dyn App> {
    Box::new(NodeJS {})
  }

  fn call_signature_for_other_app(&self, version: &Version, platform: Platform) -> CallSignature {
    let os = super::nodejs::os_text(platform.os);
    let cpu = super::nodejs::cpu_text(platform.cpu);
    let sep = path::MAIN_SEPARATOR;
    let executable = self.executable_filename(platform);
    format!("node-v{version}-{os}-{cpu}{sep}bin{sep}{executable}",)
  }
}
