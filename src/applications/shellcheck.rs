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
      url: format!(
        "https://github.com/{ORG}/{REPO}/releases/download/v{version}/shellcheck-v{version}.{os}.{cpu}.{ext}",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu),
        ext = ext_text(platform.os),
      ),
      path_in_archive: format!(
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

fn ext_text(os: Os) -> &'static str {
  match os {
    Os::Linux | Os::MacOS => "tar.xz",
    Os::Windows => "zip",
  }
}

fn cpu_text(cpu: Cpu) -> &'static str {
  match cpu {
    Cpu::Arm64 => "aarch64",
    Cpu::Intel64 => "x86_64",
  }
}

fn os_text(os: Os) -> &'static str {
  match os {
    Os::Linux => "linux",
    Os::MacOS => "darwin",
    Os::Windows => "windows",
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"version: (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::shellcheck::ShellCheck;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (ShellCheck {}).install_methods(
        &Version::from("0.9.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = vec![Method::DownloadArchive {
        url: S("https://github.com/koalaman/shellcheck/releases/download/v0.9.0/shellcheck-v0.9.0.linux.x86_64.tar.xz"),
        path_in_archive: S("shellcheck-v0.9.0/shellcheck"),
      }];
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (ShellCheck {}).install_methods(
        &Version::from("0.10.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = vec![Method::DownloadArchive {
        url: S("https://github.com/koalaman/shellcheck/releases/download/v0.10.0/shellcheck-v0.10.0.darwin.aarch64.tar.xz"),
        path_in_archive: S("shellcheck-v0.10.0/shellcheck"),
      }];
      assert_eq!(have, want);
    }
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
