use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::subshell::Executable;
use crate::{Output, Result};
use const_format::formatcp;

pub struct Goda {}

const ORG: &str = "loov";
const REPO: &str = "goda";

impl App for Goda {
    fn name(&self) -> AppName {
        AppName::from("goda")
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        github_releases::latest(ORG, REPO, output)
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        github_releases::versions(ORG, REPO, amount, output)
    }

    fn analyze_executable(&self, executable: &Executable) -> AnalyzeResult {
        if !identify(&executable.run_output("help")) {
            return AnalyzeResult::NotIdentified;
        }
        // as of 0.5.7 goda has no way to determine the version of the installed executable
        AnalyzeResult::IdentifiedButUnknownVersion
    }

    fn install_methods(&self) -> Vec<install::Method> {
        vec![Method::CompileGoSource(self)]
    }
}

impl install::CompileGoSource for Goda {
    fn import_path(&self, version: &Version) -> String {
        format!("github.com/{ORG}/{REPO}@v{version}")
    }
}

fn identify(output: &str) -> bool {
    output.contains("Print dependency graph")
}
