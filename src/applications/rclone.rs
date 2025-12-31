use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::Log;
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};

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
        bin_folder: BinFolder::Root,
      }],
    }
  }

  fn latest_installable_version(&self, _log: Log) -> Result<Version> {
    // TODO: remove this file once govulncheck is integrated into golangci-lint
    Ok(Version::from("1.1.4"))
  }

  fn installable_versions(&self, _amount: usize, _log: Log) -> Result<Vec<Version>> {
    Ok(vec![Version::from("1.1.4"), Version::from("1.1.3")])
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("Govulncheck reports known vulnerabilities in dependencies") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // govulncheck does not display the version of the installed executable
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
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
