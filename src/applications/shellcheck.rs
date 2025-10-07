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
pub(crate) struct ShellCheck {}

const ORG: &str = "koalaman";
const REPO: &str = "shellcheck";

impl AppDefinition for ShellCheck {
  fn name(&self) -> ApplicationName {
    "shellcheck".into()
  }

  fn homepage(&self) -> &'static str {
    "https://www.shellcheck.net"
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
      Os::Linux | Os::MacOS => "tar.xz",
      Os::Windows => "zip",
    };
    RunMethod::ThisApp {
      install_methods: vec![Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/shellcheck-v{version}.{os}.{cpu}.{ext}"),
        bin_folder: BinFolder::Subfolder {
          path: PathBuf::from(format!("shellcheck-v{version}")),
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
    let output = executable.run_output(&["--version"], log)?;
    if !output.contains("ShellCheck - shell script analysis tool") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&output) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"version: (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::shellcheck::ShellCheck;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;
    use std::path::PathBuf;

    #[test]
    fn linux_arm() {
      let have = (ShellCheck {}).run_method(
        &Version::from("0.9.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: S("https://github.com/koalaman/shellcheck/releases/download/v0.9.0/shellcheck-v0.9.0.linux.x86_64.tar.xz"),
          bin_folder: BinFolder::Subfolder {
            path: PathBuf::from("shellcheck-v0.9.0"),
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (ShellCheck {}).run_method(
        &Version::from("0.10.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: S("https://github.com/koalaman/shellcheck/releases/download/v0.10.0/shellcheck-v0.10.0.darwin.aarch64.tar.xz"),
          bin_folder: BinFolder::Subfolder {
            path: PathBuf::from("shellcheck-v0.10.0"),
          },
        }],
      };
      assert_eq!(have, want);
    }
  }

  mod extract_version {
    use crate::UserError;
    use crate::applications::shellcheck::extract_version;

    #[test]
    fn success() {
      let give = "
ShellCheck - shell script analysis tool
version: 0.9.0
license: GNU General Public License, version 3
website: https://www.shellcheck.net";
      assert_eq!(extract_version(give), Ok("0.9.0"));
    }

    #[test]
    fn other() {
      assert_eq!(extract_version("other"), Err(UserError::RegexDoesntMatch));
    }
  }
}
