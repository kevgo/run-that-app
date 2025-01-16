use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{regexp, Log};
use const_format::formatcp;

pub struct MdBook {}

const ORG: &str = "rust-lang";
const REPO: &str = "mdBook";

impl App for MdBook {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("mdbook")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn install_methods(&self, version: &Version, platform: Platform) -> Vec<installation::Method> {
    vec![
      Method::DownloadArchive {
        url: archive_url(version, platform),
        path_in_archive: self.executable_filename(platform),
      },
      Method::CompileRustSource {
        crate_name: "mdbook",
        filepath: format!("bin/{}", self.executable_filename(platform)),
      },
    ]
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Creates a book from markdown files") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output("-V", log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn archive_url(version: &Version, platform: Platform) -> String {
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
  format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/mdbook-v{version}-{cpu}-{os}.{ext}")
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"mdbook v(\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::configuration::Version;
  use crate::platform::{Cpu, Os, Platform};
  use crate::UserError;

  #[test]
  fn archive_url() {
    let platform = Platform {
      os: Os::Linux,
      cpu: Cpu::Intel64,
    };
    let have = super::archive_url(&Version::from("0.4.37"), platform);
    let want = "https://github.com/rust-lang/mdBook/releases/download/v0.4.37/mdbook-v0.4.37-x86_64-unknown-linux-gnu.tar.gz";
    assert_eq!(have, want);
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("mdbook v0.4.37"), Ok("0.4.37"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
