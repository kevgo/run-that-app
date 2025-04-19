use super::{AnalyzeResult, AppDefinition};
use crate::configuration::Version;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::{Log, regexp};
use big_s::S;
use const_format::formatcp;

pub(crate) struct MdBookLinkCheck {}

const ORG: &str = "Michael-F-Bryan";
const REPO: &str = "mdbook-linkcheck";

impl AppDefinition for MdBookLinkCheck {
  fn name(&self) -> &'static str {
    "mdbook-linkcheck"
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let os = match platform.os {
      Os::Linux => "unknown-linux-gnu",
      Os::MacOS => "apple-darwin",
      Os::Windows => "pc-windows-msvc",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "aarch64",
      Cpu::Intel64 => "x86_64",
    };
    RunMethod::ThisApp {
      install_methods: vec![
        Method::DownloadArchive {
          url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/mdbook-linkcheck.{cpu}-{os}.zip"),
          bin_folder: BinFolder::Root,
        },
        Method::CompileRustSource {
          crate_name: "mdbook-linkcheck",
          bin_folder: BinFolder::Subfolder { path: S("bin") },
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
    if !output.contains("mdbook-linkcheck") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output(&["-V"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }

  fn clone(&self) -> Box<dyn AppDefinition> {
    Box::new(Self {})
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"mdbook-linkcheck (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::mdbook_linkcheck::MdBookLinkCheck;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (MdBookLinkCheck {}).run_method(
        &Version::from("0.7.8"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: S("https://github.com/Michael-F-Bryan/mdbook-linkcheck/releases/download/v0.7.8/mdbook-linkcheck.x86_64-apple-darwin.zip"),
            bin_folder: BinFolder::Root,
          },
          Method::CompileRustSource {
            crate_name: "mdbook-linkcheck",
            bin_folder: BinFolder::Subfolder { path: S("bin") },
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (MdBookLinkCheck {}).run_method(
        &Version::from("0.7.8"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: S("https://github.com/Michael-F-Bryan/mdbook-linkcheck/releases/download/v0.7.8/mdbook-linkcheck.x86_64-pc-windows-msvc.zip"),
            bin_folder: BinFolder::Root,
          },
          Method::CompileRustSource {
            crate_name: "mdbook-linkcheck",
            bin_folder: BinFolder::Subfolder { path: S("bin") },
          },
        ],
      };
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("mdbook-linkcheck 0.7.7"), Ok("0.7.7"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
