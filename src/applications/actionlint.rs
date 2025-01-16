use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{regexp, Log};
use const_format::formatcp;

pub struct ActionLint {}

const ORG: &str = "rhysd";
const REPO: &str = "actionlint";

impl App for ActionLint {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("actionlint")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://{ORG}.github.io/{REPO}")
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn install_methods(&self, version: &Version, platform: Platform) -> Vec<installation::Method> {
    vec![
      Method::DownloadArchive {
        url: archive_url(version, platform),
        executable_path_in_archive: self.executable_filename(platform),
      },
      Method::CompileGoSource {
        import_path: format!("github.com/{ORG}/{REPO}/cmd/actionlint@v{version}"),
      },
    ]
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("actionlint is a linter for GitHub Actions workflow files") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    let output = executable.run_output("--version", log)?;
    match extract_version(&output) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::NotIdentified { output }),
    }
  }
}

fn archive_url(version: &Version, platform: Platform) -> String {
  let cpu = match platform.cpu {
    Cpu::Arm64 => "arm64",
    Cpu::Intel64 => "amd64",
  };
  let os = match platform.os {
    Os::Linux => "linux",
    Os::MacOS => "darwin",
    Os::Windows => "windows",
  };
  let ext = match platform.os {
    Os::Linux | Os::MacOS => "tar.gz",
    Os::Windows => "zip",
  };
  format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/actionlint_{version}_{os}_{cpu}.{ext}",)
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"(\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::configuration::Version;
  use crate::platform::{Cpu, Os, Platform};
  use crate::UserError;

  #[test]
  fn download_url() {
    let platform = Platform {
      os: Os::Linux,
      cpu: Cpu::Arm64,
    };
    let have = super::archive_url(&Version::from("1.6.26"), platform);
    let want = "https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_linux_arm64.tar.gz";
    assert_eq!(have, want);
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("1.6.27"), Ok("1.6.27"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
