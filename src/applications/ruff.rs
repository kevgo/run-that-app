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
pub(crate) struct Ruff {}

const ORG: &str = "astral-sh";
const REPO: &str = "ruff";

impl AppDefinition for Ruff {
  fn name(&self) -> ApplicationName {
    "ruff".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/astral-sh/ruff")
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
    let ext = match platform.os {
      Os::Linux | Os::MacOS => "tar.gz",
      Os::Windows => "zip",
    };
    RunMethod::ThisApp {
      install_methods: vec![Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/{version}/ruff-{cpu}-{os}.{ext}").into(),
        bin_folder: BinFolder::Subfolder {
          path: format!("ruff-{cpu}-{os}").into(),
        },
      }],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, "v", log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, "v", log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("An extremely fast Python linter and code formatter") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"ruff (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::ruff::Ruff;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn macos_arm() {
      let have = (Ruff {}).run_method(
        &Version::from("0.15.5"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/astral-sh/ruff/releases/download/0.15.5/ruff-aarch64-apple-darwin.tar.gz".into(),
          bin_folder: BinFolder::Subfolder {
            path: "ruff-aarch64-apple-darwin".into(),
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_arm() {
      let have = (Ruff {}).run_method(
        &Version::from("0.15.5"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/astral-sh/ruff/releases/download/0.15.5/ruff-aarch64-unknown-linux-gnu.tar.gz".into(),
          bin_folder: BinFolder::Subfolder {
            path: "ruff-aarch64-unknown-linux-gnu".into(),
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Ruff {}).run_method(
        &Version::from("0.15.5"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://github.com/astral-sh/ruff/releases/download/0.15.5/ruff-x86_64-pc-windows-msvc.zip".into(),
          bin_folder: BinFolder::Subfolder {
            path: "ruff-x86_64-pc-windows-msvc".into(),
          },
        }],
      };
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("ruff 0.15.5"), Ok("0.15.5"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
