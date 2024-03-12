use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::regexp;
use crate::subshell::Executable;
use crate::{Output, Result};
use std::path;

pub struct Gh {}

const ORG: &str = "cli";
const REPO: &str = "cli";

impl App for Gh {
    fn name(&self) -> AppName {
        AppName::from("gh")
    }

    fn homepage(&self) -> &'static str {
        "https://cli.github.com"
    }

    fn install_methods(&self) -> Vec<install::Method> {
        vec![Method::DownloadArchive(self)]
        // installation from source seems more involved, see https://github.com/cli/cli/blob/trunk/docs/source.md
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

impl install::DownloadArchive for Gh {
    fn archive_url(&self, version: &Version, platform: Platform) -> String {
        format!(
            "https://github.com/{ORG}/{REPO}/releases/download/v{version}/gh_{version}_{os}_{cpu}.{ext}",
            os = os_text(platform.os),
            cpu = cpu_text(platform.cpu),
            ext = ext_text(platform.os)
        )
    }

    fn executable_path_in_archive(&self, version: &Version, platform: Platform) -> String {
        format!(
            "gh_{version}_{os}_{cpu}{sep}bin{sep}{filename}",
            os = os_text(platform.os),
            cpu = cpu_text(platform.cpu),
            sep = path::MAIN_SEPARATOR,
            filename = self.executable_filename(platform)
        )
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "arm64",
        Cpu::Intel64 => "amd64",
    }
}

fn ext_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "tar.gz",
        Os::Windows | Os::MacOS => "zip",
    }
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"gh version (\d+\.\d+\.\d+)")
}

fn identify(output: &str) -> bool {
    output.contains("Work seamlessly with GitHub from the command line")
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "linux",
        Os::MacOS => "macOS",
        Os::Windows => "windows",
    }
}

#[cfg(test)]
mod tests {
    use crate::config::Version;
    use crate::install::DownloadArchive;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn archive_url() {
        let gh = super::Gh {};
        let platform = Platform { os: Os::Linux, cpu: Cpu::Intel64 };
        let have = gh.archive_url(&Version::from("2.39.1"), platform);
        let want = "https://github.com/cli/cli/releases/download/v2.39.1/gh_2.39.1_linux_amd64.tar.gz";
        assert_eq!(have, want);
    }

    mod executable_locations {
        use crate::config::Version;
        use crate::install::DownloadArchive;
        use crate::platform::{Cpu, Os, Platform};
        use big_s::S;

        #[test]
        fn executable_locations() {
            let gh = super::super::Gh {};
            let version = Version::from("1.2.3");
            let platform = Platform { os: Os::Linux, cpu: Cpu::Arm64 };
            let have = gh.executable_path_in_archive(&version, platform);
            #[cfg(unix)]
            let want = S("gh_1.2.3_linux_arm64/bin/gh");
            #[cfg(windows)]
            let want = S("gh_1.2.3_linux_arm64\\bin\\gh");
            assert_eq!(have, want);
        }
    }

    mod extract_version {
        use super::super::extract_version;

        #[test]
        fn success() {
            let output = "
gh version 2.45.0 (2024-03-04)
https://github.com/cli/cli/releases/tag/v2.45.0
";
            assert_eq!(extract_version(output), Some("2.45.0"));
        }

        #[test]
        fn other() {
            assert_eq!(extract_version("other"), None);
        }
    }
}
