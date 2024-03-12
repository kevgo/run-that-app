use super::go::Go;
use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::install::{Method, OtherAppFolder};
use crate::platform::Platform;
use crate::subshell::Executable;
use crate::{install, Output, Result};
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

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        self.app_to_install().latest_installable_version(output)
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        self.app_to_install().installable_versions(amount, output)
    }

    fn analyze_executable(&self, executable: &Executable) -> AnalyzeResult {
        if !identify(&executable.run_output("-h")) {
            return AnalyzeResult::NotIdentified;
        }
        // TODO: return the version of Go here
        AnalyzeResult::IdentifiedButUnknownVersion
    }
}

impl install::OtherAppFolder for Gofmt {
    fn app_to_install(&self) -> Box<dyn App> {
        Box::new(Go {})
    }

    fn executable_location(&self, version: &Version, platform: Platform) -> String {
        format!("bin{}{}", path::MAIN_SEPARATOR, self.executable_filename(platform))
    }
}

fn identify(output: &str) -> bool {
    output.contains("report all errors (not just the first 10 on different lines)")
}
