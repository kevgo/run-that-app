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
pub(crate) struct Conc {}

const ORG: &str = "kevgo";
const REPO: &str = "conc";

impl AppDefinition for Conc {
  fn name(&self) -> ApplicationName {
    "conc".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm_64",
      Cpu::Intel64 => "intel_64",
    };
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "macos",
      Os::Windows => "windows",
    };
    let ext = match platform.os {
      Os::Linux | Os::MacOS => "tar.gz",
      Os::Windows => "zip",
    };
    RunMethod::ThisApp {
      install_methods: vec![Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/conc_{os}_{cpu}.{ext}").into(),
        bin_folder: BinFolder::Root,
      }],
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
    if !output.contains("Conc runs commands concurrently and returns the first non-zero exit code it encounters.") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"conc (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::conc::Conc;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let have = (Conc {}).run_method(
        &Version::from("0.1.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/kevgo/conc/releases/download/v0.1.0/conc_linux_arm_64.tar.gz".into(),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Conc {}).run_method(
        &Version::from("0.1.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/kevgo/conc/releases/download/v0.1.0/conc_windows_intel_64.zip".into(),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("conc 0.6.2"), Ok("0.6.2"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
