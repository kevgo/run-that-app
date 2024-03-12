use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::regexp;
use crate::subshell::Executable;
use crate::{Output, Result};
use const_format::formatcp;

pub struct GolangCiLint {}

const ORG: &str = "golangci";
const REPO: &str = "golangci-lint";

impl App for GolangCiLint {
    fn name(&self) -> AppName {
        AppName::from("golangci-lint")
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn install_methods(&self) -> Vec<install::Method> {
        // install from source not recommended, see https://golangci-lint.run/usage/install/#install-from-source
        vec![Method::DownloadArchive(self)]
    }

    fn latest_installable_version(&self, output: Output) -> Result<Version> {
        github_releases::latest(ORG, REPO, output)
    }

    fn installable_versions(&self, amount: usize, output: Output) -> Result<Vec<Version>> {
        github_releases::versions(ORG, REPO, amount, output)
    }

    fn analyze_executable(&self, executable: &Executable) -> AnalyzeResult {
        match extract_version(&executable.run_output("--version")) {
            Some(version) => AnalyzeResult::IdentifiedWithVersion(version.into()),
            None => AnalyzeResult::IdentifiedButUnknownVersion,
        }
    }
}

impl install::DownloadArchive for GolangCiLint {
    fn archive_url(&self, version: &Version, platform: Platform) -> String {
        let os = match platform.os {
            Os::Linux => "linux",
            Os::MacOS => "darwin",
            Os::Windows => "windows",
        };
        let cpu = match platform.cpu {
            Cpu::Arm64 => "arm64",
            Cpu::Intel64 => "amd64",
        };
        let ext = match platform.os {
            Os::Linux | Os::MacOS => "tar.gz",
            Os::Windows => "zip",
        };
        format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/golangci-lint-{version}-{os}-{cpu}.{ext}")
    }

    fn executable_path_in_archive(&self, _version: &Version, platform: Platform) -> String {
        self.executable_filename(platform)
    }
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"golangci-lint has version (\d+\.\d+\.\d+) built with")
}

#[cfg(test)]
mod tests {
    use crate::config::Version;
    use crate::install::DownloadArchive;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn archive_url() {
        let golangci_lint = super::GolangCiLint {};
        let platform = Platform { os: Os::MacOS, cpu: Cpu::Arm64 };
        let have = golangci_lint.archive_url(&Version::from("1.55.2"), platform);
        let want = "https://github.com/golangci/golangci-lint/releases/download/v1.55.2/golangci-lint-1.55.2-darwin-arm64.tar.gz";
        assert_eq!(have, want);
    }

    #[test]
    fn extract_version() {
        assert_eq!(
            super::extract_version("golangci-lint has version 1.56.2 built with go1.22.0 from 58a724a0 on 2024-02-15T18:01:51Z"),
            Some("1.56.2")
        );
        assert_eq!(super::extract_version("other"), None);
    }
}
