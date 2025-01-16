use super::go::Go;
use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::installation::Method;
use crate::platform::Platform;
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{installation, Log};
use std::path;

pub struct Gofmt {}

impl App for Gofmt {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("gofmt")
  }

  fn homepage(&self) -> &'static str {
    "https://go.dev"
  }

  fn install_methods(&self, _version: &Version, platform: Platform) -> Vec<installation::Method> {
    vec![Method::ExecutableInAnotherApp {
      app_to_install: app_to_install(),
      executable_path: executable_path_in_other_app_yard(&self.executable_filename(platform)),
    }]
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    app_to_install().latest_installable_version(log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    app_to_install().installable_versions(amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("report all errors (not just the first 10 on different lines)") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // TODO: return the version of Go here
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

fn app_to_install() -> Box<dyn App> {
  Box::new(Go {})
}

fn executable_path_in_other_app_yard(executable_filename: &str) -> String {
  format!("go{sep}bin{sep}{executable_filename}", sep = path::MAIN_SEPARATOR,)
}
