use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{regexp, Log};
use std::path;

pub struct Gh {}

const ORG: &str = "cli";
const REPO: &str = "cli";

impl App for Gh {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("gh")
  }

  fn homepage(&self) -> &'static str {
    "https://cli.github.com"
  }

  fn install_methods(&self, version: &Version, platform: Platform) -> Vec<installation::Method> {
    vec![Method::DownloadArchive {
      url: archive_url(version, platform),
      executable_path: executable_path_in_archive(version, platform, self.executable_filename(platform)),
    }]
    // installation from source seems more involved, see https://github.com/cli/cli/blob/trunk/docs/source.md
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Work seamlessly with GitHub from the command line") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output("--version", log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn archive_url(version: &Version, platform: Platform) -> String {
  format!(
    "https://github.com/{ORG}/{REPO}/releases/download/v{version}/gh_{version}_{os}_{cpu}.{ext}",
    os = os_text(platform.os),
    cpu = cpu_text(platform.cpu),
    ext = ext_text(platform.os)
  )
}

fn executable_path_in_archive(version: &Version, platform: Platform, executable_filename: String) -> String {
  let os = os_text(platform.os);
  let cpu = cpu_text(platform.cpu);
  let sep = path::MAIN_SEPARATOR;
  let filename = executable_filename;
  match platform.os {
    Os::Windows => format!("bin{sep}{filename}"),
    Os::Linux | Os::MacOS => format!("gh_{version}_{os}_{cpu}{sep}bin{sep}{filename}",),
  }
}

fn cpu_text(cpu: Cpu) -> &'static str {
  match cpu {
    Cpu::Arm64 => "arm64",
    Cpu::Intel64 => "amd64",
  }
}

fn ext_text(os: Os) -> &'static str {
  match os {
    Os::Linux => "tar.gz",
    Os::Windows | Os::MacOS => "zip",
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"gh version (\d+\.\d+\.\d+)")
}

fn os_text(os: Os) -> &'static str {
  match os {
    Os::Linux => "linux",
    Os::MacOS => "macOS",
    Os::Windows => "windows",
  }
}

#[cfg(test)]
mod tests {
  use crate::configuration::Version;
  use crate::platform::{Cpu, Os, Platform};

  #[test]
  fn archive_url() {
    let platform = Platform { os: Os::Linux, cpu: Cpu::Intel64 };
    let have = super::archive_url(&Version::from("2.39.1"), platform);
    let want = "https://github.com/cli/cli/releases/download/v2.39.1/gh_2.39.1_linux_amd64.tar.gz";
    assert_eq!(have, want);
  }

  mod executable_locations {
    use crate::configuration::Version;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn executable_locations() {
      let version = Version::from("1.2.3");
      let platform = Platform { os: Os::Linux, cpu: Cpu::Arm64 };
      let have = super::super::executable_path_in_archive(&version, platform, S("gh"));
      #[cfg(unix)]
      let want = S("gh_1.2.3_linux_arm64/bin/gh");
      #[cfg(windows)]
      let want = S("gh_1.2.3_linux_arm64\\bin\\gh");
      assert_eq!(have, want);
    }
  }

  mod extract_version {
    use super::super::extract_version;
    use crate::UserError;

    #[test]
    fn success() {
      let output = "
gh version 2.45.0 (2024-03-04)
https://github.com/cli/cli/releases/tag/v2.45.0
";
      assert_eq!(extract_version(output), Ok("2.45.0"));
    }

    #[test]
    fn other() {
      assert_eq!(extract_version("other"), Err(UserError::RegexDoesntMatch));
    }
  }
}
