use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::Log;
use const_format::formatcp;

pub struct Ireturn {}

const ORG: &str = "butuzov";
const REPO: &str = "ireturn";

impl App for Ireturn {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("ireturn")
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
      Method::CompileGoSource {
        import_path: format!("github.com/{ORG}/{REPO}/cmd/ireturn@v{version}"),
      },
    ]
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

fn archive_url(version: &Version, platform: Platform) -> String {
  format!(
    "https://github.com/{ORG}/{REPO}/releases/download/v{version}/ireturn_{os}_{cpu}.{ext}",
    os = os_text(platform.os),
    cpu = cpu_text(platform.cpu),
    ext = ext_text(platform.os)
  )
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
  use crate::configuration::Version;
  use crate::platform::{Cpu, Os, Platform};

  #[test]
  fn archive_url() {
    let platform = Platform {
      os: Os::Linux,
      cpu: Cpu::Intel64,
    };
    let have = super::archive_url(&Version::from("0.3.0"), platform);
    let want = "https://github.com/butuzov/ireturn/releases/download/v0.3.0/ireturn_linux_x86_64.tar.gz";
    assert_eq!(have, want);
  }
}
