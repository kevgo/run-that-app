use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::Method;
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{install, regexp, Log};
use const_format::formatcp;

pub struct Shfmt {}

const ORG: &str = "mvdan";
const REPO: &str = "sh";

impl App for Shfmt {
    fn name(&self) -> AppName {
        AppName::from("shfmt")
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn install_methods(&self) -> Vec<install::Method> {
        vec![Method::DownloadExecutable(self), Method::CompileGoSource(self)]
    }

    fn latest_installable_version(&self, log: Log) -> Result<Version> {
        github_releases::latest(ORG, REPO, log)
    }

    fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
        github_releases::versions(ORG, REPO, amount, log)
    }

    fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
        let output = executable.run_output("-h", log)?;
        if !identify(&output) {
            return Ok(AnalyzeResult::NotIdentified { output });
        }
        match extract_version(&executable.run_output("--version", log)?) {
            Some(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
            None => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
        }
    }
}

impl install::DownloadExecutable for Shfmt {
    fn download_url(&self, version: &Version, platform: Platform) -> String {
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
            Os::Linux | Os::MacOS => "",
            Os::Windows => ".exe",
        };
        format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/shfmt_v{version}_{os}_{cpu}{ext}")
    }
}

impl install::CompileGoSource for Shfmt {
    fn import_path(&self, version: &Version) -> String {
        format!("mvdan.cc/sh/v3/cmd/shfmt@v{version}")
    }
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"^v(\d+\.\d+\.\d+)$")
}

fn identify(output: &str) -> bool {
    output.contains("shfmt formats shell programs")
}

#[cfg(test)]
mod tests {
    use crate::config::Version;
    use crate::install::DownloadExecutable;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn artifact_url() {
        let shfmt = super::Shfmt {};
        let platform = Platform {
            os: Os::MacOS,
            cpu: Cpu::Arm64,
        };
        let have = shfmt.download_url(&Version::from("3.7.0"), platform);
        let want = "https://github.com/mvdan/sh/releases/download/v3.7.0/shfmt_v3.7.0_darwin_arm64";
        assert_eq!(have, want);
    }

    #[test]
    fn extract_version() {
        assert_eq!(super::extract_version("v3.7.0"), Some("3.7.0"));
        assert_eq!(super::extract_version("3.7.0"), None);
        assert_eq!(super::extract_version("other"), None);
    }
}
