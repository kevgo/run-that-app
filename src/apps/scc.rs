use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{compile_go, download_archive, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::subshell::Executable;
use crate::{install, regexp};
use crate::{Output, Result};
use const_format::formatcp;

pub struct Scc {}

const ORG: &str = "boyter";
const REPO: &str = "scc";

impl App for Scc {
    fn name(&self) -> AppName {
        AppName::from("scc")
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn install_methods(&self) -> Vec<install::Method> {
        vec![Method::DownloadArchive(self), Method::CompileGoSource(self)]
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        github_releases::latest(ORG, REPO, output)
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        github_releases::versions(ORG, REPO, amount, output)
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

impl download_archive::Data for Scc {
    fn archive_url(&self, version: &Version, platform: Platform) -> String {
        let os = match platform.os {
            Os::Linux => "Linux",
            Os::MacOS => "Darwin",
            Os::Windows => "Windows",
        };
        let cpu = match platform.cpu {
            Cpu::Arm64 => "arm64",
            Cpu::Intel64 => "x86_64",
        };
        format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/scc_{version}_{os}_{cpu}.tar.gz")
    }
}

impl compile_go::Data for Scc {
    fn import_path(&self, version: &Version) -> String {
        format!("github.com/{ORG}/{REPO}/v3@v{version}")
    }
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"scc version (\d+\.\d+\.\d+)")
}

fn identify(output: &str) -> bool {
    output.contains("Count lines of code in a directory with complexity estimation")
}

#[cfg(test)]
mod tests {
    use crate::config::Version;
    use crate::install::download_archive::Data;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn archive_url() {
        let scc = super::Scc {};
        let platform = Platform { os: Os::MacOS, cpu: Cpu::Arm64 };
        let have = scc.archive_url(&Version::from("3.1.0"), platform);
        let want = "https://github.com/boyter/scc/releases/download/v3.1.0/scc_3.1.0_Darwin_arm64.tar.gz";
        assert_eq!(have, want);
    }

    #[test]
    fn extract_version() {
        assert_eq!(super::extract_version("scc version 3.2.0"), Some("3.2.0"));
        assert_eq!(super::extract_version("other"), None);
    }
}
