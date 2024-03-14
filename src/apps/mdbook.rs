use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::regexp;
use crate::subshell::Executable;
use crate::{LogFn, Result};
use const_format::formatcp;

pub struct MdBook {}

const ORG: &str = "rust-lang";
const REPO: &str = "mdBook";

impl App for MdBook {
    fn name(&self) -> AppName {
        AppName::from("mdbook")
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn install_methods(&self) -> Vec<install::Method> {
        vec![Method::DownloadArchive(self), Method::CompileRustSource(self)]
    }

    fn latest_installable_version(&self, log: LogFn) -> Result<Version> {
        github_releases::latest(ORG, REPO, log)
    }

    fn installable_versions(&self, amount: usize, log: LogFn) -> Result<Vec<Version>> {
        github_releases::versions(ORG, REPO, amount, log)
    }

    fn analyze_executable(&self, executable: &Executable, log: LogFn) -> Result<AnalyzeResult> {
        let output = executable.run_output("-h", log)?;
        if !identify(&output) {
            return Ok(AnalyzeResult::NotIdentified { output });
        }
        match extract_version(&executable.run_output("-V", log)?) {
            Some(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
            None => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
        }
    }
}

impl install::DownloadArchive for MdBook {
    fn archive_url(&self, version: &Version, platform: Platform) -> String {
        let os = match platform.os {
            Os::Linux => "unknown-linux-gnu",
            Os::MacOS => "apple-darwin",
            Os::Windows => "pc-windows-msvc",
        };
        let cpu = match platform.cpu {
            Cpu::Arm64 => "aarch64",
            Cpu::Intel64 => "x86_64",
        };
        format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/mdbook-v{version}-{cpu}-{os}.tar.gz")
    }

    fn executable_path_in_archive(&self, _version: &Version, platform: Platform) -> String {
        self.executable_filename(platform)
    }
}

impl install::CompileRustSource for MdBook {
    fn crate_name(&self) -> &'static str {
        "mdbook"
    }
}

fn extract_version(output: &str) -> Option<&str> {
    regexp::first_capture(output, r"mdbook v(\d+\.\d+\.\d+)")
}

fn identify(output: &str) -> bool {
    output.contains("Creates a book from markdown files")
}

#[cfg(test)]
mod tests {
    use crate::config::Version;
    use crate::install::DownloadArchive;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn archive_url() {
        let mdbook = super::MdBook {};
        let platform = Platform { os: Os::Linux, cpu: Cpu::Intel64 };
        let have = mdbook.archive_url(&Version::from("0.4.37"), platform);
        let want = "https://github.com/rust-lang/mdBook/releases/download/v0.4.37/mdbook-v0.4.37-x86_64-unknown-linux-gnu.tar.gz";
        assert_eq!(have, want);
    }

    #[test]
    fn extract_version() {
        assert_eq!(super::extract_version("mdbook v0.4.37"), Some("0.4.37"));
        assert_eq!(super::extract_version("other"), None);
    }
}
