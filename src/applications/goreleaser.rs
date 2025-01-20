use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::run::{self, Executable};
use crate::{regexp, Log};

pub struct Goreleaser {}

const ORG: &str = "goreleaser";
const REPO: &str = "goreleaser";

impl App for Goreleaser {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("goreleaser")
  }

  fn homepage(&self) -> &'static str {
    "https://goreleaser.com"
  }

  fn run_method(&self, version: &Version, platform: Platform) -> run::Method {
    let os = match platform.os {
      Os::Linux => "Linux",
      Os::MacOS => "Darwin",
      Os::Windows => "Windows",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "x86_64",
    };
    let ext = match platform.os {
      Os::Linux | Os::MacOS => "tar.gz",
      Os::Windows => "zip",
    };
    run::Method::ThisApp {
      install_methods: vec![Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/goreleaser_{os}_{cpu}.{ext}"),
        bin_folders: vec![],
      }],
    }
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-v", log)?;
    if !output.contains("https://goreleaser.com") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&output) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }

  fn clone(&self) -> Box<dyn App> {
    Box::new(Self {})
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"GitVersion:\s*(\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    #[cfg(unix)]
    fn linux_arm() {
      use crate::applications::goreleaser::Goreleaser;
      use crate::run;

      let have = (Goreleaser {}).run_method(
        &Version::from("1.22.1"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = run::Method::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: S("https://github.com/goreleaser/goreleaser/releases/download/v1.22.1/goreleaser_Darwin_arm64.tar.gz"),
          bin_folders: vec![],
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn windows_intel() {
      use crate::applications::goreleaser::Goreleaser;

      let have = (Goreleaser {}).install_methods(
        &Version::from("1.22.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = vec![Method::DownloadArchive {
        url: S("https://github.com/goreleaser/goreleaser/releases/download/v1.22.1/goreleaser_Windows_x86_64.zip"),
        bin_folders: vec![],
      }];
      assert_eq!(have, want);
    }
  }

  mod extract_version {
    use super::super::extract_version;
    use crate::UserError;

    #[test]
    fn version_1() {
      let output = r"
  ____       ____      _
 / ___| ___ |  _ \ ___| | ___  __ _ ___  ___ _ __
| |  _ / _ \| |_) / _ \ |/ _ \/ _` / __|/ _ \ '__|
| |_| | (_) |  _ <  __/ |  __/ (_| \__ \  __/ |
 \____|\___/|_| \_\___|_|\___|\__,_|___/\___|_|
goreleaser: Deliver Go Binaries as fast and easily as possible
https://goreleaser.com

GitVersion:    1.24.0
GitCommit:     00c2ff733758f63201811c337f8d043e8fcc9d58
GitTreeState:  false
BuildDate:     2024-02-05T12:18:01Z
BuiltBy:       goreleaser
GoVersion:     go1.21.6
Compiler:      gc
ModuleSum:     h1:jsoS5T2CvPKOyECPATAo8hCvUaX8ok4iAq9m5Zyl1L0=
Platform:      linux/arm64
";
      assert_eq!(extract_version(output), Ok("1.24.0"));
    }

    #[test]
    fn other() {
      assert_eq!(extract_version("other"), Err(UserError::RegexDoesntMatch));
    }
  }
}
