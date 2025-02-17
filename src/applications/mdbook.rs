use super::{AnalyzeResult, AppDefinition};
use crate::configuration::Version;
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::run::ExecutablePath;
use crate::{regexp, run, Log};
use big_s::S;
use const_format::formatcp;

pub(crate) struct MdBook {}

const ORG: &str = "rust-lang";
const REPO: &str = "mdBook";

impl AppDefinition for MdBook {
  fn name(&self) -> &'static str {
    "mdbook"
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> run::Method {
    let os = match platform.os {
      Os::Linux => "unknown-linux-gnu",
      Os::MacOS => "apple-darwin",
      Os::Windows => "pc-windows-msvc",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "aarch64",
      Cpu::Intel64 => "x86_64",
    };
    let ext = match platform.os {
      Os::Linux | Os::MacOS => "tar.gz",
      Os::Windows => "zip",
    };
    run::Method::ThisApp {
      install_methods: vec![
        Method::DownloadArchive {
          url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/mdbook-v{version}-{cpu}-{os}.{ext}"),
          bin_folder: BinFolder::Root,
        },
        Method::CompileRustSource {
          crate_name: "mdbook",
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

  fn analyze_executable(&self, executable: &ExecutablePath, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("Creates a book from markdown files") {
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
  regexp::first_capture(output, r"mdbook v(\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod install_methods {
    use crate::applications::mdbook::MdBook;
    use crate::applications::AppDefinition;
    use crate::configuration::Version;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};
    use crate::run;
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (MdBook {}).run_method(
        &Version::from("0.4.37"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = run::Method::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: S("https://github.com/rust-lang/mdBook/releases/download/v0.4.37/mdbook-v0.4.37-x86_64-unknown-linux-gnu.tar.gz"),
            bin_folder: BinFolder::Root,
          },
          Method::CompileRustSource {
            crate_name: "mdbook",
            bin_folder: BinFolder::Subfolder { path: S("bin") },
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (MdBook {}).run_method(
        &Version::from("0.4.37"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = run::Method::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: S("https://github.com/rust-lang/mdBook/releases/download/v0.4.37/mdbook-v0.4.37-x86_64-pc-windows-msvc.zip"),
            bin_folder: BinFolder::Root,
          },
          Method::CompileRustSource {
            crate_name: "mdbook",
            bin_folder: BinFolder::Subfolder { path: S("bin") },
          },
        ],
      };
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("mdbook v0.4.37"), Ok("0.4.37"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
