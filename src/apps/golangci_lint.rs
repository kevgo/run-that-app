use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{regexp, Log};
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

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    match extract_version(&executable.run_output("--version", log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

impl install::DownloadArchive for GolangCiLint {
  fn archive_url(&self, version: &Version, platform: Platform) -> String {
    format!(
      "https://github.com/{ORG}/{REPO}/releases/download/v{version}/golangci-lint-{version}-{os}-{cpu}.{ext}",
      os = os_text(platform.os),
      cpu = cpu_text(platform.cpu),
      ext = ext_text(platform.os)
    )
  }

  fn executable_path_in_archive(&self, version: &Version, platform: Platform) -> String {
    format!(
      "golangci-lint-{version}-{os}-{cpu}{sep}{executable}",
      executable = self.executable_filename(platform),
      os = os_text(platform.os),
      cpu = cpu_text(platform.cpu),
      sep = std::path::MAIN_SEPARATOR
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
    Os::Linux | Os::MacOS => "tar.gz",
    Os::Windows => "zip",
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"golangci-lint has version (\d+\.\d+\.\d+) built with")
}

fn os_text(os: Os) -> &'static str {
  match os {
    Os::Linux => "linux",
    Os::MacOS => "darwin",
    Os::Windows => "windows",
  }
}

#[cfg(test)]
mod tests {
  use crate::apps::UserError;
  use crate::config::Version;
  use crate::install::DownloadArchive;
  use crate::platform::{Cpu, Os, Platform};
  use big_s::S;

  #[test]
  fn archive_url() {
    let golangci_lint = super::GolangCiLint {};
    let platform = Platform {
      os: Os::MacOS,
      cpu: Cpu::Arm64,
    };
    let have = golangci_lint.archive_url(&Version::from("1.55.2"), platform);
    let want = "https://github.com/golangci/golangci-lint/releases/download/v1.55.2/golangci-lint-1.55.2-darwin-arm64.tar.gz";
    assert_eq!(have, want);
  }

  #[test]
  fn extract_version() {
    assert_eq!(
      super::extract_version("golangci-lint has version 1.56.2 built with go1.22.0 from 58a724a0 on 2024-02-15T18:01:51Z"),
      Ok("1.56.2")
    );
    assert_eq!(
      super::extract_version("other"),
      Err(UserError::RegexHasNoCaptures {
        regex: S(r"golangci-lint has version (\d+\.\d+\.\d+) built with")
      })
    );
  }
}
