use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::Log;
use const_format::formatcp;

pub struct Ireturn {}

const ORG: &str = "butuzov";
const REPO: &str = "ireturn";

impl App for Ireturn {
  fn name(&self) -> AppName {
    AppName::from("ireturn")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn install_methods(&self) -> Vec<install::Method> {
    vec![Method::DownloadArchive(self), Method::CompileGoSource(self)]
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("ireturn: Accept Interfaces, Return Concrete Types") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

impl install::DownloadArchive for Ireturn {
  fn archive_url(&self, version: &Version, platform: Platform) -> String {
    format!(
      "https://github.com/{ORG}/{REPO}/releases/download/v{version}/ireturn_{os}_{cpu}.{ext}",
      os = os_text(platform.os),
      cpu = cpu_text(platform.cpu),
      ext = ext_text(platform.os)
    )
  }

  fn executable_path_in_archive(&self, _version: &Version, platform: Platform) -> String {
    self.executable_filename(platform)
  }
}

impl install::CompileGoSource for Ireturn {
  fn import_path(&self, version: &Version) -> String {
    format!("github.com/{ORG}/{REPO}/cmd/ireturn@{version}")
  }
}

fn cpu_text(cpu: Cpu) -> &'static str {
  match cpu {
    Cpu::Arm64 => "arm64",
    Cpu::Intel64 => "x86_64",
  }
}

fn ext_text(os: Os) -> &'static str {
  match os {
    Os::Linux | Os::MacOS => "tar.gz",
    Os::Windows => "zip",
  }
}

fn os_text(os: Os) -> &'static str {
  match os {
    Os::Linux => "linux",
    Os::MacOS => "darwin",
    Os::Windows => "windows",
  }
}

#[cfg(test)]
mod tests {
  use crate::config::Version;
  use crate::install::DownloadArchive;
  use crate::platform::{Cpu, Os, Platform};

  #[test]
  fn archive_url() {
    let ireturn = super::Ireturn {};
    let platform = Platform {
      os: Os::Linux,
      cpu: Cpu::Intel64,
    };
    let have = ireturn.archive_url(&Version::from("0.3.0"), platform);
    let want = "https://github.com/butuzov/ireturn/releases/download/v0.3.0/ireturn_linux_x86_64.tar.gz";
    assert_eq!(have, want);
  }
}
