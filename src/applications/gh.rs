use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, strings};
use std::path;

#[derive(Clone)]
pub(crate) struct Gh {}

const ORG: &str = "cli";
const REPO: &str = "cli";
const TAG_PREFIX: &str = "v";

impl AppDefinition for Gh {
  fn name(&self) -> ApplicationName {
    "gh".into()
  }

  fn homepage(&self) -> &'static str {
    "https://cli.github.com"
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "macOS",
      Os::Windows => "windows",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "amd64",
    };
    let ext = match platform.os {
      Os::Linux => "tar.gz",
      Os::Windows | Os::MacOS => "zip",
    };
    let sep = path::MAIN_SEPARATOR;
    RunMethod::ThisApp {
      install_methods: vec![Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/{TAG_PREFIX}{version}/gh_{version}_{os}_{cpu}.{ext}").into(),
        bin_folder: BinFolder::Subfolders {
          options: vec!["bin".into(), format!("gh_{version}_{os}_{cpu}{sep}bin").into()],
        },
      }],
    }
    // installation from source seems more involved, see https://github.com/cli/cli/blob/trunk/docs/source.md
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, TAG_PREFIX, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, TAG_PREFIX, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("Work seamlessly with GitHub from the command line") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  strings::first_capture(output, r"gh version (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {

  mod run_method {
    use std::path::MAIN_SEPARATOR;

    use crate::applications::AppDefinition;
    use crate::applications::gh::Gh;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};

    fn want(url: &str, gh_extract_folder: &str) -> RunMethod {
      let sep = MAIN_SEPARATOR;
      RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: url.into(),
          bin_folder: BinFolder::Subfolders {
            options: vec![
              "bin".into(),
              format!("gh_2.39.1_{gh_extract_folder}{sep}bin").into(),
            ],
          },
        }],
      }
    }

    #[test]
    fn linux_arm() {
      assert_eq!(
        (Gh {}).run_method(
          &Version::from("2.39.1"),
          Platform {
            os: Os::Linux,
            cpu: Cpu::Arm64,
          },
        ),
        want(
          "https://github.com/cli/cli/releases/download/v2.39.1/gh_2.39.1_linux_arm64.tar.gz",
          "linux_arm64",
        ),
      );
    }

    #[test]
    fn linux_intel() {
      assert_eq!(
        (Gh {}).run_method(
          &Version::from("2.39.1"),
          Platform {
            os: Os::Linux,
            cpu: Cpu::Intel64,
          },
        ),
        want(
          "https://github.com/cli/cli/releases/download/v2.39.1/gh_2.39.1_linux_amd64.tar.gz",
          "linux_amd64",
        ),
      );
    }

    #[test]
    fn macos_arm() {
      assert_eq!(
        (Gh {}).run_method(
          &Version::from("2.39.1"),
          Platform {
            os: Os::MacOS,
            cpu: Cpu::Arm64,
          },
        ),
        want(
          "https://github.com/cli/cli/releases/download/v2.39.1/gh_2.39.1_macOS_arm64.zip",
          "macOS_arm64",
        ),
      );
    }

    #[test]
    fn macos_intel() {
      assert_eq!(
        (Gh {}).run_method(
          &Version::from("2.39.1"),
          Platform {
            os: Os::MacOS,
            cpu: Cpu::Intel64,
          },
        ),
        want(
          "https://github.com/cli/cli/releases/download/v2.39.1/gh_2.39.1_macOS_amd64.zip",
          "macOS_amd64",
        ),
      );
    }

    #[test]
    fn windows_arm() {
      assert_eq!(
        (Gh {}).run_method(
          &Version::from("2.39.1"),
          Platform {
            os: Os::Windows,
            cpu: Cpu::Arm64,
          },
        ),
        want(
          "https://github.com/cli/cli/releases/download/v2.39.1/gh_2.39.1_windows_arm64.zip",
          "windows_arm64",
        ),
      );
    }

    #[test]
    fn windows_intel() {
      assert_eq!(
        (Gh {}).run_method(
          &Version::from("2.39.1"),
          Platform {
            os: Os::Windows,
            cpu: Cpu::Intel64,
          },
        ),
        want(
          "https://github.com/cli/cli/releases/download/v2.39.1/gh_2.39.1_windows_amd64.zip",
          "windows_amd64",
        ),
      );
    }
  }

  mod extract_version {
    use super::super::extract_version;
    use crate::UserError;

    #[test]
    fn success() {
      let output = "
gh version 2.45.0 (2024-03-04)
https://github.com/cli/cli/releases/tag/v2.45.0
";
      assert_eq!(extract_version(output), Ok("2.45.0"));
    }

    #[test]
    fn other() {
      assert_eq!(extract_version("other"), Err(UserError::RegexDoesntMatch));
    }
  }
}
