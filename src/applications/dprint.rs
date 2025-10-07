use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, regexp};
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct Dprint {}

const ORG: &str = "dprint";
const REPO: &str = "dprint";

impl AppDefinition for Dprint {
  fn name(&self) -> ApplicationName {
    "dprint".into()
  }

  fn homepage(&self) -> &'static str {
    "https://dprint.dev"
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let cpu = match platform.cpu {
      Cpu::Arm64 => "aarch64",
      Cpu::Intel64 => "x86_64",
    };
    let os = match platform.os {
      Os::Linux => "unknown-linux-gnu",
      Os::MacOS => "apple-darwin",
      Os::Windows => "pc-windows-msvc",
    };
    RunMethod::ThisApp {
      install_methods: vec![
        Method::DownloadArchive {
          url: format!("https://github.com/{ORG}/{REPO}/releases/download/{version}/dprint-{cpu}-{os}.zip"),
          bin_folder: BinFolder::Root,
        },
        Method::CompileRustSource {
          crate_name: "dprint",
          bin_folder: BinFolder::Subfolder { path: PathBuf::from("bin") },
        },
      ],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("Auto-formats source code based on the specified plugins") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"dprint (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use std::path::PathBuf;

    use super::super::Dprint;
    use crate::applications::AppDefinition;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn macos_arm() {
      let have = (Dprint {}).run_method(
        &Version::from("0.48.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: S("https://github.com/dprint/dprint/releases/download/0.48.0/dprint-aarch64-apple-darwin.zip"),
            bin_folder: BinFolder::Root,
          },
          Method::CompileRustSource {
            crate_name: "dprint",
            bin_folder: BinFolder::Subfolder { path: PathBuf::from("bin") },
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_arm() {
      let have = (Dprint {}).run_method(
        &Version::from("0.48.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: S("https://github.com/dprint/dprint/releases/download/0.48.0/dprint-aarch64-unknown-linux-gnu.zip"),
            bin_folder: BinFolder::Root,
          },
          Method::CompileRustSource {
            crate_name: "dprint",
            bin_folder: BinFolder::Subfolder { path: PathBuf::from("bin") },
          },
        ],
      };
      assert_eq!(have, want);
    }
  }
}
