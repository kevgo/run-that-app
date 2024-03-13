use super::go::Go;
use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::install::{Method, ViaAnotherApp};
use crate::platform::Platform;
use crate::subshell::Executable;
use crate::{install, Log, Result};
use std::path;

pub struct Gofmt {}

impl App for Gofmt {
    fn name(&self) -> AppName {
        AppName::from("gofmt")
    }

    fn homepage(&self) -> &'static str {
        "https://go.dev"
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
        let output = executable.run_output("-h", log)?;
        if !identify(&output) {
            return Ok(AnalyzeResult::NotIdentified { output });
        }
        // TODO: return the version of Go here
        Ok(AnalyzeResult::IdentifiedButUnknownVersion)
    }
}

impl install::ViaAnotherApp for Gofmt {
    fn app_to_install(&self) -> Box<dyn App> {
        Box::new(Go {})
    }

    fn executable_path_in_other_app_yard(&self, _version: &Version, platform: Platform) -> String {
        let sep = path::MAIN_SEPARATOR;
        format!("go{sep}bin{sep}{}", self.executable_filename(platform))
    }
}

fn identify(output: &str) -> bool {
    output.contains("report all errors (not just the first 10 on different lines)")
}
