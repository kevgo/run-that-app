use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::regexp;
use crate::subshell::Executable;
use crate::{Output, Result};

pub struct ShellCheck {}

const ORG: &str = "koalaman";
const REPO: &str = "shellcheck";

impl App for ShellCheck {
    fn name(&self) -> AppName {
        AppName::from("shellcheck")
    }

    fn homepage(&self) -> &'static str {
        "https://www.shellcheck.net"
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
        let output = executable.run_output("--version");
        if !identify(&output) {
            return AnalyzeResult::NotIdentified;
        }
        match extract_version(&output) {
            Some(version) => AnalyzeResult::IdentifiedWithVersion(version.into()),
            None => AnalyzeResult::IdentifiedButUnknownVersion,
        }
    }
}

impl install::DownloadArchive for ShellCheck {
    fn archive_url(&self, version: &Version, platform: Platform) -> String {
        let os = match platform.os {
            Os::Linux => "linux",
            Os::MacOS => "darwin",
            Os::Windows => "windows",
        };
        let cpu = match platform.cpu {
            Cpu::Arm64 => "aarch64",
            Cpu::Intel64 => "x86_64",
        };
        let ext = match platform.os {
            Os::Linux | Os::MacOS => "tar.xz",
            Os::Windows => "zip",
        };
        format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/shellcheck-v{version}.{os}.{cpu}.{ext}")
    }

    fn executable_path_in_archive(&self, _version: &Version, platform: Platform) -> String {
        format!("{}/{}", self.name().as_str(), self.executable_filename(platform))
    }
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"version: (\d+\.\d+\.\d+)")
}

fn identify(output: &str) -> bool {
    output.contains("ShellCheck - shell script analysis tool")
}

#[cfg(test)]
mod tests {
    use crate::config::Version;
    use crate::install::DownloadArchive;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn archive_url() {
        let shellcheck = super::ShellCheck {};
        let platform = Platform { os: Os::Linux, cpu: Cpu::Intel64 };
        let have = shellcheck.archive_url(&Version::from("0.9.0"), platform);
        let want = "https://github.com/koalaman/shellcheck/releases/download/v0.9.0/shellcheck-v0.9.0.linux.x86_64.tar.xz";
        assert_eq!(have, want);
    }

    mod extract_version {
        use super::super::extract_version;

        #[test]
        fn success() {
            let give = "
ShellCheck - shell script analysis tool
version: 0.9.0
license: GNU General Public License, version 3
website: https://www.shellcheck.net";
            assert_eq!(extract_version(give), Some("0.9.0"));
        }

        #[test]
        fn other() {
            assert_eq!(extract_version("other"), None);
        }
    }
}
