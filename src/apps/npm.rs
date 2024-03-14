use super::nodejs::NodeJS;
use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::install::{self, Method, ViaAnotherApp};
use crate::platform::Platform;
use crate::subshell::Executable;
use crate::{Log, Result};
use std::path;

pub struct Npm {}

impl App for Npm {
    fn name(&self) -> AppName {
        AppName::from("npm")
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
        if !identify(&output) {
            return Ok(AnalyzeResult::NotIdentified { output });
        }
        // npm is versioned together with NodeJS, the actual version of npm is therefore not relevant here.
        Ok(AnalyzeResult::IdentifiedButUnknownVersion)
    }
}

impl install::ViaAnotherApp for Npm {
    fn app_to_install(&self) -> Box<dyn App> {
        Box::new(NodeJS {})
    }

    fn executable_path_in_other_app_yard(&self, version: &Version, platform: Platform) -> String {
        let os = super::nodejs::os_text(platform.os);
        let cpu = super::nodejs::cpu_text(platform.cpu);
        let sep = path::MAIN_SEPARATOR;
        format!("node-v{version}-{os}-{cpu}{sep}bin{sep}{}", self.executable_filename(platform))
    }
}

fn identify(output: &str) -> bool {
    output.contains("javascript package manager")
}

#[cfg(test)]
mod tests {

    #[test]
    fn extract_version() {
        assert_eq!(super::extract_version("10.2.4"), Some("10.2.4"));
        assert_eq!(super::extract_version("other"), None);
    }
}
