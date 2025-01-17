use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{regexp, Log};
use const_format::formatcp;

pub struct GolangCiLint {}

const ORG: &str = "golangci";
const REPO: &str = "golangci-lint";

impl App for GolangCiLint {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("golangci-lint")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn install_methods(&self, version: &Version, platform: Platform) -> Vec<installation::Method> {
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
      Os::Linux | Os::MacOS => "tar.gz",
      Os::Windows => "zip",
    };
    let sep = std::path::MAIN_SEPARATOR;
    let filename = self.executable_filename(platform);
    // install from source not recommended, see https://golangci-lint.run/usage/install/#install-from-source
    vec![Method::DownloadArchive {
      url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/golangci-lint-{version}-{os}-{cpu}.{ext}",),
      path_in_archive: format!("golangci-lint-{version}-{os}-{cpu}{sep}{filename}"),
    }]
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

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"golangci-lint has version (\d+\.\d+\.\d+) built with")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod install_methods {
    use crate::applications::golangci_lint::GolangCiLint;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    #[cfg(unix)]
    fn linux_arm() {
      let have = (GolangCiLint {}).install_methods(
        &Version::from("1.55.2"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = vec![Method::DownloadArchive {
        url: S("https://github.com/golangci/golangci-lint/releases/download/v1.55.2/golangci-lint-1.55.2-darwin-arm64.tar.gz"),
        path_in_archive: S("golangci-lint-1.55.2-darwin-arm64/golangci-lint"),
      }];
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn windows_intel() {
      let have = (GolangCiLint {}).install_methods(
        &Version::from("1.55.2"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = vec![Method::DownloadArchive {
        url: S("https://github.com/golangci/golangci-lint/releases/download/v1.55.2/golangci-lint-1.55.2-windows-amd64.zip"),
        path_in_archive: S("golangci-lint-1.55.2-windows-amd64\\golangci-lint.exe"),
      }];
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(
      super::extract_version("golangci-lint has version 1.56.2 built with go1.22.0 from 58a724a0 on 2024-02-15T18:01:51Z"),
      Ok("1.56.2")
    );
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
