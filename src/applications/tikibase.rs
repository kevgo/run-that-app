use super::{AnalyzeResult, AppDefinition};
use crate::configuration::Version;
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::executables::Executable;
use crate::{regexp, executables, Log};
use const_format::formatcp;

pub(crate) struct Tikibase {}

const ORG: &str = "kevgo";
const REPO: &str = "tikibase";

impl AppDefinition for Tikibase {
  fn name(&self) -> &'static str {
    "tikibase"
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> executables::Method {
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "intel64",
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
    executables::Method::ThisApp {
      install_methods: vec![Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/tikibase_{os}_{cpu}.{ext}"),
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
    if !output.contains("Linter for Markdown-based knowledge databases") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
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
  regexp::first_capture(output, r"tikibase (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod install_methods {
    use crate::applications::tikibase::Tikibase;
    use crate::applications::AppDefinition;
    use crate::configuration::Version;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};
    use crate::executables;
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (Tikibase {}).run_method(
        &Version::from("0.6.2"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = executables::Method::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: S("https://github.com/kevgo/tikibase/releases/download/v0.6.2/tikibase_macos_arm64.tar.gz"),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Tikibase {}).run_method(
        &Version::from("0.6.2"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = executables::Method::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: S("https://github.com/kevgo/tikibase/releases/download/v0.6.2/tikibase_windows_intel64.zip"),
          bin_folder: BinFolder::Root,
        }],
      };
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("tikibase 0.6.2"), Ok("0.6.2"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
