use super::nodejs::NodeJS;
use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::install::{self, Method};
use crate::platform::Platform;
use crate::regexp;
use crate::subshell::Executable;
use crate::{Output, Result};
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

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        (NodeJS {}).latest_installable_version(output)
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        (NodeJS {}).installable_versions(amount, output)
    }

    fn analyze_executable(&self, executable: &Executable) -> AnalyzeResult {
        if !identify(&executable.run_output("-h")) {
            return AnalyzeResult::NotIdentified;
        }
        match extract_version(&executable.run_output("--version")) {
            Some(version) => AnalyzeResult::IdentifiedWithVersion(version.into()),
            None => AnalyzeResult::IdentifiedButUnknownVersion,
        }
    }
}

impl install::InstallAnotherApp for Npx {
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
