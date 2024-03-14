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

    fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
        let output = executable.run_output("-h", log)?;
        if !identify(&output) {
            return Ok(AnalyzeResult::NotIdentified { output });
        }
        match extract_version(&executable.run_output("--version", log)?) {
            Some(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
            None => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
        }
    }
}

impl install::ViaAnotherApp for Npx {
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
