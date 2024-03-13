use super::nodejs::NodeJS;
use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::install::{self, Method};
use crate::platform::Platform;
use crate::regexp;
use crate::subshell::Executable;
use crate::{Log, Result};
use std::path;

pub struct Npx {}

impl App for Npx {
    fn name(&self) -> AppName {
        AppName::from("npx")
    }

    fn homepage(&self) -> &'static str {
        "https://www.npmjs.com"
    }

    fn install_methods(&self) -> Vec<install::Method> {
        vec![Method::InstallAnotherApp(self)]
    }

    fn latest_installable_version(&self, log: Log) -> Result<Version> {
        (NodeJS {}).latest_installable_version(log)
    }

    fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
        (NodeJS {}).installable_versions(amount, log)
    }

    fn analyze_executable(&self, executable: &Executable) -> AnalyzeResult {
        let output = executable.run_output("-h");
        if !identify(&output) {
            return AnalyzeResult::NotIdentified { output };
        }
        match extract_version(&executable.run_output("--version")) {
            Some(version) => AnalyzeResult::IdentifiedWithVersion(version.into()),
            None => AnalyzeResult::IdentifiedButUnknownVersion,
        }
    }
}

impl install::ViaAnotherApp for Npx {
    fn app_to_install(&self) -> Box<dyn App> {
        Box::new(NodeJS {})
    }

    fn executable_path_in_other_app_yard(&self, _version: &Version, platform: Platform) -> String {
        format!("bin{}{}", path::MAIN_SEPARATOR, self.executable_filename(platform))
    }
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"(\d+\.\d+\.\d+)")
}

fn identify(output: &str) -> bool {
    output.contains("Run a command from a local or remote npm package")
}

#[cfg(test)]
mod tests {

    #[test]
    fn extract_version() {
        assert_eq!(super::extract_version("10.2.4"), Some("10.2.4"));
        assert_eq!(super::extract_version("other"), None);
    }
}
