use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{regexp, Log};

pub struct ShellCheck {}

const ORG: &str = "koalaman";
const REPO: &str = "shellcheck";

impl App for ShellCheck {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("shellcheck")
  }

  fn homepage(&self) -> &'static str {
    "https://www.shellcheck.net"
  }

  fn install_methods(&self, version: &Version, platform: Platform) -> Vec<installation::Method> {
    vec![Method::DownloadArchive {
      url: archive_url(version, platform),
      executable_path: format!(
        "shellcheck-v{version}{sep}{executable}",
        sep = std::path::MAIN_SEPARATOR,
        executable = self.executable_filename(platform)
      ),
    }]
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("--version", log)?;
    if !output.contains("ShellCheck - shell script analysis tool") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&output) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn archive_url(version: &Version, platform: Platform) -> String {
  let os = match platform.os {
    Os::Linux => "linux",
    Os::MacOS => "darwin",
    Os::Windows => "windows",
  };
  let cpu = match platform.cpu {
    Cpu::Arm64 => "aarch64",
    Cpu::Intel64 => "x86_64",
  };
  let ext = match platform.os {
    Os::Linux | Os::MacOS => "tar.xz",
    Os::Windows => "zip",
  };
  format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/shellcheck-v{version}.{os}.{cpu}.{ext}")
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"version: (\d+\.\d+\.\d+)")
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
    let have = super::archive_url(&Version::from("0.9.0"), platform);
    let want = "https://github.com/koalaman/shellcheck/releases/download/v0.9.0/shellcheck-v0.9.0.linux.x86_64.tar.xz";
    assert_eq!(have, want);
  }

  mod extract_version {
    use super::super::extract_version;
    use crate::UserError;

    #[test]
    fn success() {
      let give = "
ShellCheck - shell script analysis tool
version: 0.9.0
license: GNU General Public License, version 3
website: https://www.shellcheck.net";
      assert_eq!(extract_version(give), Ok("0.9.0"));
    }

    #[test]
    fn other() {
      assert_eq!(extract_version("other"), Err(UserError::RegexDoesntMatch));
    }
  }
}
