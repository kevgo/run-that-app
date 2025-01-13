use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{regexp, Log};
use const_format::formatcp;

pub struct MdBookLinkCheck {}

const ORG: &str = "Michael-F-Bryan";
const REPO: &str = "mdbook-linkcheck";

impl App for MdBookLinkCheck {
  fn name(&self) -> AppName {
    AppName::from("mdbook-linkcheck")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn install_methods(&self) -> Vec<install::Method> {
    vec![Method::DownloadArchive(self), Method::CompileRustSource(self)]
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("mdbook-linkcheck") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output("-V", log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

impl install::DownloadArchive for MdBookLinkCheck {
  fn archive_url(&self, version: &Version, platform: Platform) -> String {
    let os = match platform.os {
      Os::Linux => "unknown-linux-gnu",
      Os::MacOS => "apple-darwin",
      Os::Windows => "pc-windows-msvc",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "aarch64",
      Cpu::Intel64 => "x86_64",
    };
    format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/mdbook-linkcheck.{cpu}-{os}.zip")
  }

  fn executable_path_in_archive(&self, _version: &Version, platform: Platform) -> String {
    self.executable_filename(platform)
  }
}

impl install::CompileRustSource for MdBookLinkCheck {
  fn crate_name(&self) -> &'static str {
    "mdbook-linkcheck"
  }

  fn executable_path_in_folder(&self, platform: Platform) -> String {
    self.executable_filename(platform)
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"mdbook-linkcheck (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::apps::UserError;
  use crate::config::Version;
  use crate::install::DownloadArchive;
  use crate::platform::{Cpu, Os, Platform};

  #[test]
  fn archive_url() {
    let mdbook = super::MdBookLinkCheck {};
    let platform = Platform {
      os: Os::MacOS,
      cpu: Cpu::Intel64,
    };
    let have = mdbook.archive_url(&Version::from("0.7.8"), platform);
    let want = "https://github.com/Michael-F-Bryan/mdbook-linkcheck/releases/download/v0.7.8/mdbook-linkcheck.x86_64-apple-darwin.zip";
    assert_eq!(have, want);
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("mdbook-linkcheck 0.7.7"), Ok("0.7.7"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
