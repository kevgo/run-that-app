use super::nodejs::NodeJS;
use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::platform::{Os, Platform};
use crate::regexp;
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};

pub struct Npx {}

impl App for Npx {
    fn name(&self) -> AppName {
        AppName::from("npx")
    }

    fn executable_filepath(&self, platform: Platform) -> String {
        match platform.os {
            Os::Linux | Os::MacOS => "bin/npx".into(),
            Os::Windows => "bin\\npx.exe".into(),
        }
    }

    fn homepage(&self) -> &'static str {
        "https://www.npmjs.com"
    }

    fn install(&self, version: &Version, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        let nodejs = NodeJS {};
        nodejs.install(version, platform, yard, output)?;
        let executable_path = yard.app_folder(&nodejs.name(), version)?.join(self.executable_filepath(platform));
        Ok(Some(Executable(executable_path)))
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        (NodeJS {}).latest_installable_version(output)
    }

    fn load(&self, version: &Version, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(&(NodeJS {}).name(), version, &self.executable_filepath(platform))
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
