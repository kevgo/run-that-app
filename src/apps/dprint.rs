use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::regexp;
use crate::subshell::Executable;
use crate::{Output, Result};

pub struct Dprint {}

const ORG: &str = "dprint";
const REPO: &str = "dprint";

impl App for Dprint {
    fn name(&self) -> AppName {
        AppName::from("dprint")
    }

    fn homepage(&self) -> &'static str {
        "https://dprint.dev"
    }

    fn install_methods(&self) -> Vec<crate::install::Method> {
        vec![Method::DownloadArchive(self), Method::CompileRustSource(self)]
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
        match extract_version(&executable.run_output("--version")) {
            Some(version) => AnalyzeResult::IdentifiedWithVersion(version.into()),
            None => AnalyzeResult::IdentifiedButUnknownVersion,
        }
    }
}

impl install::DownloadArchive for Dprint {
    fn archive_url(&self, version: &Version, platform: Platform) -> String {
        let cpu = match platform.cpu {
            Cpu::Arm64 => "aarch64",
            Cpu::Intel64 => "x86_64",
        };
        let os = match platform.os {
            Os::Linux => "unknown-linux-gnu",
            Os::MacOS => "apple-darwin",
            Os::Windows => "pc-windows-msvc",
        };
        format!("https://github.com/{ORG}/{REPO}/releases/download/{version}/dprint-{cpu}-{os}.zip")
    }
}

impl install::CompileRust for Dprint {
    fn crate_name(&self) -> &'static str {
        "dprint"
    }
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"dprint (\d+\.\d+\.\d+)")
}

fn identify(output: &str) -> bool {
    output.contains("Auto-formats source code based on the specified plugins")
}

#[cfg(test)]
mod tests {

    mod archive_url {
        use crate::config::Version;
        use crate::install::DownloadArchive;
        use crate::platform::{Cpu, Os, Platform};

        #[test]
        fn mac_arm() {
            let dprint = super::super::Dprint {};
            let platform = Platform { os: Os::MacOS, cpu: Cpu::Arm64 };
            let have = dprint.archive_url(&Version::from("0.43.0"), platform);
            let want = "https://github.com/dprint/dprint/releases/download/0.43.0/dprint-aarch64-apple-darwin.zip";
            assert_eq!(have, want);
        }

        #[test]
        fn linux_arm() {
            let dprint = super::super::Dprint {};
            let platform = Platform { os: Os::Linux, cpu: Cpu::Arm64 };
            let have = dprint.archive_url(&Version::from("0.43.1"), platform);
            let want = "https://github.com/dprint/dprint/releases/download/0.43.1/dprint-aarch64-unknown-linux-gnu.zip";
            assert_eq!(have, want);
        }
    }

    #[test]
    fn extract_version() {
        assert_eq!(super::extract_version("dprint 0.45.0"), Some("0.45.0"));
        assert_eq!(super::extract_version("other"), None);
    }
}
