use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{regexp, Log};
use std::path;

pub struct NodeJS {}

pub const ORG: &str = "nodejs";
pub const REPO: &str = "node";

impl App for NodeJS {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("node")
  }

  fn homepage(&self) -> &'static str {
    "https://nodejs.org"
  }

  fn install_methods(&self, version: &Version, platform: Platform) -> Vec<installation::Method> {
    vec![Method::DownloadArchive {
      archive_url: archive_url(version, platform),
      executable_path_in_archive: executable_path_in_archive(version, platform, self.executable_filename(platform)),
    }]
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Documentation can be found at https://nodejs.org") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output("--version", log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn archive_url(version: &Version, platform: Platform) -> String {
  let ext = match platform.os {
    Os::Linux => "tar.xz",
    Os::MacOS => "tar.gz",
    Os::Windows => "zip",
  };
  format!(
    "https://nodejs.org/dist/v{version}/node-v{version}-{os}-{cpu}.{ext}",
    os = os_text(platform.os),
    cpu = cpu_text(platform.cpu),
  )
}

fn executable_path_in_archive(version: &Version, platform: Platform, executable_filename: String) -> String {
  let os = os_text(platform.os);
  let cpu = cpu_text(platform.cpu);
  let sep = path::MAIN_SEPARATOR;
  match platform.os {
    Os::Windows => format!("node-v{version}-{os}-{cpu}{sep}{executable_filename}"),
    Os::Linux | Os::MacOS => format!("node-v{version}-{os}-{cpu}{sep}bin{sep}{executable_filename}"),
  }
}

pub fn cpu_text(cpu: Cpu) -> &'static str {
  match cpu {
    Cpu::Arm64 => "arm64",
    Cpu::Intel64 => "x64",
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"v(\d+\.\d+\.\d+)")
}

pub fn os_text(os: Os) -> &'static str {
  match os {
    Os::Linux => "linux",
    Os::MacOS => "darwin",
    Os::Windows => "win",
  }
}

#[cfg(test)]
mod tests {
  use crate::configuration::Version;
  use crate::platform::{Cpu, Os, Platform};
  use crate::UserError;

  #[test]
  fn archive_url() {
    let platform = Platform {
      os: Os::MacOS,
      cpu: Cpu::Arm64,
    };
    let have = super::archive_url(&Version::from("20.10.0"), platform);
    let want = "https://nodejs.org/dist/v20.10.0/node-v20.10.0-darwin-arm64.tar.gz";
    assert_eq!(have, want);
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("v10.2.4"), Ok("10.2.4"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
