use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::regexp;
use crate::subshell::Executable;
use crate::Log;
use const_format::formatcp;

pub struct ActionLint {}

const ORG: &str = "rhysd";
const REPO: &str = "actionlint";

impl App for ActionLint {
  fn name(&self) -> AppName {
    AppName::from("actionlint")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://{ORG}.github.io/{REPO}")
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn install_methods(&self) -> Vec<install::Method> {
    vec![Method::DownloadArchive(self), Method::CompileGoSource(self)]
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !identify(&output) {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    let output = executable.run_output("--version", log)?;
    match extract_version(&output) {
      Some(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      None => Ok(AnalyzeResult::NotIdentified { output }),
    }
  }
}

impl install::DownloadArchive for ActionLint {
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
    let ext = match platform.os {
      Os::Linux | Os::MacOS => "tar.gz",
      Os::Windows => "zip",
    };
    format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/actionlint_{version}_{os}_{cpu}.{ext}",)
  }

  fn executable_path_in_archive(&self, _version: &Version, platform: Platform) -> String {
    self.executable_filename(platform)
  }
}

impl install::CompileGoSource for ActionLint {
  fn import_path(&self, version: &Version) -> String {
    format!("github.com/{ORG}/{REPO}/cmd/actionlint@v{version}")
  }
}

fn extract_version(output: &str) -> Option<&str> {
  regexp::first_capture(output, r"(\d+\.\d+\.\d+)")
}

fn identify(output: &str) -> bool {
  output.contains("actionlint is a linter for GitHub Actions workflow files")
}

#[cfg(test)]
mod tests {
  use crate::config::Version;
  use crate::install::DownloadArchive;
  use crate::platform::{Cpu, Os, Platform};

  #[test]
  fn download_url() {
    let platform = Platform { os: Os::Linux, cpu: Cpu::Arm64 };
    let actionlint = super::ActionLint {};
    let have = actionlint.archive_url(&Version::from("1.6.26"), platform);
    let want = "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_linux_arm64.tar.gz";
    assert_eq!(have, want);
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("1.6.27"), Some("1.6.27"));
    assert_eq!(super::extract_version("other"), None);
  }
}
