use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::subshell::Executable;
use crate::{Output, Result};
use const_format::formatcp;

pub struct Ghokin {}

const ORG: &str = "antham";
const REPO: &str = "ghokin";

impl App for Ghokin {
    fn name(&self) -> AppName {
        AppName::from("ghokin")
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn install_methods(&self) -> Vec<crate::install::Method> {
        vec![Method::DownloadArchive(self), Method::CompileGoSource(self)]
    }
    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<Version>> {
        github_releases::versions("antham", "ghokin", amount, output)
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<Version> {
        github_releases::latest(ORG, REPO, output)
    }

    fn analyze_executable(&self, executable: &Executable) -> AnalyzeResult {
        if !identify(&executable.run_output("-h")) {
            return AnalyzeResult::NotIdentified;
        }
        // as of 3.4.0 ghokin's "version" command prints nothing
        AnalyzeResult::IdentifiedButUnknownVersion
    }
}

impl install::DownloadArchive for Ghokin {
    fn archive_url(&self, version: &Version, platform: Platform) -> String {
        let cpu = match platform.cpu {
            Cpu::Arm64 => "arm64",
            Cpu::Intel64 => "amd64",
        };
        let os = match platform.os {
            Os::Linux => "linux",
            Os::MacOS => "darwin",
            Os::Windows => "windows",
        };
        format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/ghokin_{version}_{os}_{cpu}.tar.gz")
    }

    fn executable_path_in_archive(&self, _version: &Version, platform: Platform) -> String {
        self.executable_filename(platform)
    }
}

impl install::CompileGoSource for Ghokin {
    fn import_path(&self, version: &Version) -> String {
        format!("github.com/{ORG}/{REPO}/v3@v{version}")
    }
}

fn identify(output: &str) -> bool {
    output.contains("Clean and/or apply transformation on gherkin files")
}

#[cfg(test)]
mod tests {
    mod archive_url {
        use crate::config::Version;
        use crate::install::DownloadArchive;
        use crate::platform::{Cpu, Os, Platform};

        #[test]
        fn macos_intel64() {
            let ghokin = super::super::Ghokin {};
            let platform = Platform { os: Os::MacOS, cpu: Cpu::Intel64 };
            let have = ghokin.archive_url(&Version::from("3.4.1"), platform);
            let want = "https://github.com/antham/ghokin/releases/download/v3.4.1/ghokin_3.4.1_darwin_amd64.tar.gz";
            assert_eq!(have, want);
        }

        #[test]
        fn windows_intel64() {
            let ghokin = super::super::Ghokin {};
            let platform = Platform {
                os: Os::Windows,
                cpu: Cpu::Intel64,
            };
            let have = ghokin.archive_url(&Version::from("3.4.1"), platform);
            let want = "https://github.com/antham/ghokin/releases/download/v3.4.1/ghokin_3.4.1_windows_amd64.tar.gz";
            assert_eq!(have, want);
        }
    }
}
