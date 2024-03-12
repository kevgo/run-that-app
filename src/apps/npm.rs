use super::nodejs::NodeJS;
use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::install::{Method, OtherAppFolder};
use crate::platform::Platform;
use crate::subshell::Executable;
use crate::{install, regexp};
use crate::{Output, Result};
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

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        self.app_to_install().latest_installable_version(output)
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        self.app_to_install().installable_versions(amount, output)
    }

    fn analyze_executable(&self, executable: &Executable) -> AnalyzeResult {
        if !identify(&executable.run_output_args(&["help", "npm"])) {
            return AnalyzeResult::NotIdentified;
        }
        match extract_version(&executable.run_output("--version")) {
            Some(version) => AnalyzeResult::IdentifiedWithVersion(version.into()),
            None => AnalyzeResult::IdentifiedButUnknownVersion,
        }
    }
}

impl install::OtherAppFolder for Npm {
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
