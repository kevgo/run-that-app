use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::subshell::Executable;
use crate::{Output, Result};
use const_format::formatcp;

pub struct Alphavet {}

const ORG: &str = "skx";
const REPO: &str = "alphavet";

impl App for Alphavet {
    fn name(&self) -> AppName {
        AppName::from("alphavet")
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn install_methods(&self) -> Vec<install::Method> {
        vec![Method::CompileGoSource(self)]
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        github_releases::versions(ORG, REPO, amount, output)
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        github_releases::latest(ORG, REPO, output)
    }

    fn analyze_executable(&self, executable: &Executable) -> AnalyzeResult {
        if !identify(&executable.run_output("-h")) {
            return AnalyzeResult::NotIdentified;
        }
        // as of 0.1.0 the -V switch of alphavet is broken
        AnalyzeResult::IdentifiedButUnknownVersion
    }
}

impl install::CompileGoSource for Alphavet {
    fn import_path(&self, version: &Version) -> String {
        format!("github.com/{ORG}/{REPO}/cmd/alphavet@v{version}")
    }
}

fn identify(output: &str) -> bool {
    output.contains("Checks that functions are ordered alphabetically within packages")
}
