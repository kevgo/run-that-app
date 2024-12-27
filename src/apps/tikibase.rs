use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{regexp, Log};
use const_format::formatcp;

pub struct Tikibase {}

const ORG: &str = "kevgo";
const REPO: &str = "tikibase";

impl App for Tikibase {
  fn name(&self) -> AppName {
    AppName::from("tikibase")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn install_methods(&self) -> Vec<crate::install::Method> {
    vec![Method::DownloadArchive(self)]
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Linter for Markdown-based knowledge databases") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output("--version", log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

impl install::DownloadArchive for Tikibase {
  fn archive_url(&self, version: &Version, platform: Platform) -> String {
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
    format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/tikibase_{os}_{cpu}.{ext}")
  }

  fn executable_path_in_archive(&self, _version: &Version, platform: Platform) -> String {
    self.executable_filename(platform)
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"tikibase (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::apps::UserError;

  mod archive_url {
    use crate::config::Version;
    use crate::install::DownloadArchive;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn mac_arm() {
      let tikibase = super::super::Tikibase {};
      let platform = Platform {
        os: Os::MacOS,
        cpu: Cpu::Arm64,
      };
      let have = tikibase.archive_url(&Version::from("0.6.2"), platform);
      let want = "https://github.com/kevgo/tikibase/releases/download/v0.6.2/tikibase_macos_arm64.tar.gz";
      assert_eq!(have, want);
    }

    #[test]
    fn linux_arm() {
      let tikibase = super::super::Tikibase {};
      let platform = Platform {
        os: Os::Linux,
        cpu: Cpu::Intel64,
      };
      let have = tikibase.archive_url(&Version::from("0.6.2"), platform);
      let want = "https://github.com/kevgo/tikibase/releases/download/v0.6.2/tikibase_linux_intel64.tar.gz";
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("tikibase 0.6.2"), Ok("0.6.2"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
