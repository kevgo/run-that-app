use super::{AnalyzeResult, AppDefinition};
use crate::configuration::Version;
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::run::{self, Executable};
use crate::{regexp, Log};
use const_format::formatcp;

pub(crate) struct GolangCiLint {}

const ORG: &str = "golangci";
const REPO: &str = "golangci-lint";

impl AppDefinition for GolangCiLint {
  fn name(&self) -> &'static str {
    "golangci-lint"
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> run::Method {
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
    run::Method::ThisApp { install_methods:
    // install from source not recommended, see https://golangci-lint.run/usage/install/#install-from-source
    vec![Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/golangci-lint-{version}-{os}-{cpu}.{ext}"),
        bin_folder: BinFolder::Subfolder { path: format!("golangci-lint-{version}-{os}-{cpu}")},
    }]}
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    match extract_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }

  fn clone(&self) -> Box<dyn AppDefinition> {
    Box::new(Self {})
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
    use crate::applications::AppDefinition;
    use crate::configuration::Version;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};
    use crate::run;
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (GolangCiLint {}).run_method(
        &Version::from("1.55.2"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = run::Method::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: S("https://github.com/golangci/golangci-lint/releases/download/v1.55.2/golangci-lint-1.55.2-darwin-arm64.tar.gz"),
          bin_folder: BinFolder::Subfolder {
            path: S("golangci-lint-1.55.2-darwin-arm64"),
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (GolangCiLint {}).run_method(
        &Version::from("1.55.2"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = run::Method::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: S("https://github.com/golangci/golangci-lint/releases/download/v1.55.2/golangci-lint-1.55.2-windows-amd64.zip"),
          bin_folder: BinFolder::Subfolder {
            path: S("golangci-lint-1.55.2-windows-amd64"),
          },
        }],
      };
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
