use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::Method;
use crate::platform::{Cpu, Os, Platform};
use crate::subshell::Executable;
use crate::{install, regexp};
use crate::{Output, Result};

pub struct Goreleaser {}

const ORG: &str = "goreleaser";
const REPO: &str = "goreleaser";

impl App for Goreleaser {
    fn name(&self) -> AppName {
        AppName::from("goreleaser")
    }

    fn homepage(&self) -> &'static str {
        "https://goreleaser.com"
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
        let output = &executable.run_output("-V");
        if !identify(output) {
            return AnalyzeResult::NotIdentified;
        }
        match extract_version(output) {
            Some(version) => AnalyzeResult::IdentifiedWithVersion(version.into()),
            None => AnalyzeResult::IdentifiedButUnknownVersion,
        }
    }
}

impl install::DownloadArchive for Goreleaser {
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
        let ext = match platform.os {
            Os::Linux | Os::MacOS => "tar.gz",
            Os::Windows => "zip",
        };
        format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/goreleaser_{os}_{cpu}.{ext}")
    }
}

impl install::CompileGo for Goreleaser {
    fn import_path(&self, version: &Version) -> String {
        format!("github.com/{ORG}/{REPO}@{version}")
    }
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"GitVersion:\s*(\d+\.\d+\.\d+)")
}

fn identify(output: &str) -> bool {
    output.contains("Deliver Go Binaries as fast and easily as possible")
}

#[cfg(test)]
mod tests {
    use crate::config::Version;
    use crate::install::DownloadArchive;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn archive_url() {
        let goreleaser = super::Goreleaser {};
        let platform = Platform { os: Os::MacOS, cpu: Cpu::Arm64 };
        let have = goreleaser.archive_url(&Version::from("1.22.1"), platform);
        let want = "https://github.com/goreleaser/goreleaser/releases/download/v1.22.1/goreleaser_Darwin_arm64.tar.gz";
        assert_eq!(have, want);
    }

    mod extract_version {
        use super::super::extract_version;

        #[test]
        fn success() {
            let output = r"
  ____       ____      _
 / ___| ___ |  _ \ ___| | ___  __ _ ___  ___ _ __
| |  _ / _ \| |_) / _ \ |/ _ \/ _` / __|/ _ \ '__|
| |_| | (_) |  _ <  __/ |  __/ (_| \__ \  __/ |
 \____|\___/|_| \_\___|_|\___|\__,_|___/\___|_|
goreleaser: Deliver Go Binaries as fast and easily as possible
https://goreleaser.com

GitVersion:    1.24.0
GitCommit:     00c2ff733758f63201811c337f8d043e8fcc9d58
GitTreeState:  false
BuildDate:     2024-02-05T12:18:01Z
BuiltBy:       goreleaser
GoVersion:     go1.21.6
Compiler:      gc
ModuleSum:     h1:jsoS5T2CvPKOyECPATAo8hCvUaX8ok4iAq9m5Zyl1L0=
Platform:      linux/arm64
";
            assert_eq!(extract_version(output), Some("1.24.0"));
        }

        #[test]
        fn other() {
            assert_eq!(extract_version("other"), None);
        }
    }
}
