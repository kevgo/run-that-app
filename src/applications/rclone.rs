use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, regexp};

#[derive(Clone)]
pub(crate) struct Rclone {}

const ORG: &str = "rclone";
const REPO: &str = "rclone";

impl AppDefinition for Rclone {
  fn name(&self) -> ApplicationName {
    "rclone".into()
  }

  fn homepage(&self) -> &'static str {
    "https://rclone.org"
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "osx",
      Os::Windows => "windows",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "amd64",
    };
    RunMethod::ThisApp {
      install_methods: vec![Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/rclone-v{version}-{os}-{cpu}.zip").into(),
        bin_folder: BinFolder::Subfolder {
          path: format!("rclone-v{version}-{os}-{cpu}").into(),
        },
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
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("Rclone syncs files to and from cloud storage providers") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    let output = executable.run_output(&["version"], log)?;
    match extract_version(&output) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"rclone v(\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::rclone::Rclone;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let have = (Rclone {}).run_method(
        &Version::from("1.72.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/rclone/rclone/releases/download/v1.72.1/rclone-v1.72.1-linux-arm64.zip".into(),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Rclone {}).run_method(
        &Version::from("1.72.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/rclone/rclone/releases/download/v1.72.1/rclone-v1.72.1-windows-amd64.zip".into(),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }
  }
}
