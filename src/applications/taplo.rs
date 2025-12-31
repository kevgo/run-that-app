use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, regexp};
use const_format::formatcp;

#[derive(Clone)]
pub(crate) struct Taplo {}

const ORG: &str = "tamasfe";
const REPO: &str = "taplo";

impl AppDefinition for Taplo {
  fn name(&self) -> ApplicationName {
    "taplo".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "darwin",
      Os::Windows => "windows",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "aarch64",
      Cpu::Intel64 => "x86_64",
    };
    let ext = match platform.os {
      Os::Linux | Os::MacOS => "gz",
      Os::Windows => "zip",
    };
    RunMethod::ThisApp {
      install_methods: vec![
        Method::DownloadArchive {
          url: format!("https://github.com/{ORG}/{REPO}/releases/download/{version}/taplo-{os}-{cpu}.{ext}").into(),
          bin_folder: BinFolder::Root,
        },
        Method::CompileRustSource {
          crate_name: "taplo",
          bin_folder: BinFolder::Subfolder { path: "bin".into() },
        },
      ],
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
    if !output.contains("Lint TOML documents") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output(&["-V"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"taplo (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::taplo::Taplo;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let have = (Taplo {}).run_method(
        &Version::from("0.10.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/tamasfe/taplo/releases/download/0.10.0/taplo-linux-aarch64.gz".into(),
            bin_folder: BinFolder::Root,
          },
          Method::CompileRustSource {
            crate_name: "taplo",
            bin_folder: BinFolder::Subfolder { path: "bin".into() },
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (Taplo {}).run_method(
        &Version::from("0.10.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/tamasfe/taplo/releases/download/0.10.0/taplo-darwin-aarch64.gz".into(),
            bin_folder: BinFolder::Root,
          },
          Method::CompileRustSource {
            crate_name: "taplo",
            bin_folder: BinFolder::Subfolder { path: "bin".into() },
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Taplo {}).run_method(
        &Version::from("0.10.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: "https://github.com/tamasfe/taplo/releases/download/0.10.0/taplo-windows-x86_64.zip".into(),
            bin_folder: BinFolder::Root,
          },
          Method::CompileRustSource {
            crate_name: "taplo",
            bin_folder: BinFolder::Subfolder { path: "bin".into() },
          },
        ],
      };
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("taplo 0.10.0"), Ok("0.10.0"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
