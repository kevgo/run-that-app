use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{regexp, Log};

pub struct Dprint {}

const ORG: &str = "dprint";
const REPO: &str = "dprint";

impl App for Dprint {
  fn name(&self) -> AppName {
    AppName::from("dprint")
  }

  fn homepage(&self) -> &'static str {
    "https://dprint.dev"
  }

  fn install_methods(&self) -> Vec<crate::install::Method> {
    vec![Method::DownloadArchive(self), Method::CompileRustSource(self)]
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Auto-formats source code based on the specified plugins") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output("--version", log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

impl install::DownloadArchive for Dprint {
  fn archive_url(&self, version: &Version, platform: Platform) -> String {
    let cpu = match platform.cpu {
      Cpu::Arm64 => "aarch64",
      Cpu::Intel64 => "x86_64",
    };
    let os = match platform.os {
      Os::Linux => "unknown-linux-gnu",
      Os::MacOS => "apple-darwin",
      Os::Windows => "pc-windows-msvc",
    };
    format!("https://github.com/{ORG}/{REPO}/releases/download/{version}/dprint-{cpu}-{os}.zip")
  }

  fn executable_path_in_archive(&self, _version: &Version, platform: Platform) -> String {
    self.executable_filename(platform)
  }
}

impl install::CompileRustSource for Dprint {
  fn crate_name(&self) -> &'static str {
    "dprint"
  }

  fn executable_path_in_folder(&self, platform: Platform) -> String {
    self.executable_filename(platform)
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"dprint (\d+\.\d+\.\d+)")
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
      let dprint = super::super::Dprint {};
      let platform = Platform {
        os: Os::MacOS,
        cpu: Cpu::Arm64,
      };
      let have = dprint.archive_url(&Version::from("0.43.0"), platform);
      let want = "https://github.com/dprint/dprint/releases/download/0.43.0/dprint-aarch64-apple-darwin.zip";
      assert_eq!(have, want);
    }

    #[test]
    fn linux_arm() {
      let dprint = super::super::Dprint {};
      let platform = Platform {
        os: Os::Linux,
        cpu: Cpu::Arm64,
      };
      let have = dprint.archive_url(&Version::from("0.43.1"), platform);
      let want = "https://github.com/dprint/dprint/releases/download/0.43.1/dprint-aarch64-unknown-linux-gnu.zip";
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("dprint 0.45.0"), Ok("0.45.0"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
