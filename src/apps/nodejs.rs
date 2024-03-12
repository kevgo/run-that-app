use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::Method;
use crate::platform::{Cpu, Os, Platform};
use crate::subshell::Executable;
use crate::{install, regexp};
use crate::{Output, Result};
use std::path;

pub struct NodeJS {}

pub const ORG: &str = "nodejs";
pub const REPO: &str = "node";

impl App for NodeJS {
    fn name(&self) -> AppName {
        AppName::from("node")
    }

    fn homepage(&self) -> &'static str {
        "https://nodejs.org"
    }

    fn install_methods(&self) -> Vec<install::Method> {
        vec![Method::DownloadArchive(self)]
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

impl install::DownloadArchive for NodeJS {
    fn archive_url(&self, version: &Version, platform: Platform) -> String {
        let ext = match platform.os {
            Os::Linux => "tar.xz",
            Os::MacOS => "tar.gz",
            Os::Windows => "zip",
        };
        format!(
            "https://nodejs.org/dist/v{version}/node-v{version}-{os}-{cpu}.{ext}",
            os = os_text(platform.os),
            cpu = cpu_text(platform.cpu),
        )
    }

    fn executable_location(&self, version: &Version, platform: Platform) -> String {
        format!(
            "node-v{version}-{os}-{cpu}{sep}bin{sep}{executable}",
            os = os_text(platform.os),
            cpu = cpu_text(platform.cpu),
            sep = path::MAIN_SEPARATOR,
            executable = self.executable_filename(platform)
        )
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "arm64",
        Cpu::Intel64 => "x64",
    }
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"v(\d+\.\d+\.\d+)")
}

fn identify(output: &str) -> bool {
    output.contains("Documentation can be found at https://nodejs.org")
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "linux",
        Os::MacOS => "darwin",
        Os::Windows => "win",
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Version;
    use crate::install::DownloadArchive;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn archive_url() {
        let node = super::NodeJS {};
        let platform = Platform { os: Os::MacOS, cpu: Cpu::Arm64 };
        let have = node.archive_url(&Version::from("20.10.0"), platform);
        let want = "https://nodejs.org/dist/v20.10.0/node-v20.10.0-darwin-arm64.tar.gz";
        assert_eq!(have, want);
    }

    #[test]
    fn extract_version() {
        assert_eq!(super::extract_version("v10.2.4"), Some("10.2.4"));
        assert_eq!(super::extract_version("other"), None);
    }
}
