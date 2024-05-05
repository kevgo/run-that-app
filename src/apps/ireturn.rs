use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{regexp, Log};
use const_format::formatcp;
use std::path;

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
    if !identify(&output) {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output("--version", log)?) {
      Some(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      None => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
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

  fn executable_path_in_archive(&self, version: &Version, platform: Platform) -> String {
    format!(
      "gh_{version}_{os}_{cpu}{sep}bin{sep}{filename}",
      os = os_text(platform.os),
      cpu = cpu_text(platform.cpu),
      sep = path::MAIN_SEPARATOR,
      filename = self.executable_filename(platform)
    )
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

fn extract_version(output: &str) -> Option<&str> {
  regexp::first_capture(output, r"gh version (\d+\.\d+\.\d+)")
}

fn identify(output: &str) -> bool {
  output.contains("ireturn: Accept Interfaces, Return Concrete Types")
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
    let have = ireturn.archive_url(&Version::from("2.39.1"), platform);
    let want = "https://github.com/cli/cli/releases/download/v2.39.1/gh_2.39.1_linux_amd64.tar.gz";
    assert_eq!(have, want);
  }

  mod executable_locations {
    use crate::config::Version;
    use crate::install::DownloadArchive;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn executable_locations() {
      let ireturn = super::super::Ireturn {};
      let version = Version::from("1.2.3");
      let platform = Platform {
        os: Os::Linux,
        cpu: Cpu::Arm64,
      };
      let have = ireturn.executable_path_in_archive(&version, platform);
      #[cfg(unix)]
      let want = S("gh_1.2.3_linux_arm64/bin/gh");
      #[cfg(windows)]
      let want = S("gh_1.2.3_linux_arm64\\bin\\gh");
      assert_eq!(have, want);
    }
  }

  mod extract_version {
    use super::super::extract_version;

    #[test]
    fn success() {
      let output = "
gh version 2.45.0 (2024-03-04)
https://github.com/cli/cli/releases/tag/v2.45.0
";
      assert_eq!(extract_version(output), Some("2.45.0"));
    }

    #[test]
    fn other() {
      assert_eq!(extract_version("other"), None);
    }
  }
}
