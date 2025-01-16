use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{installation, regexp, Log};
use const_format::formatcp;

pub struct Shfmt {}

const ORG: &str = "mvdan";
const REPO: &str = "sh";

impl App for Shfmt {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("shfmt")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn install_methods(&self, version: &Version, platform: Platform) -> Vec<installation::Method> {
    vec![
      Method::DownloadExecutable { url: download_url(version, platform) },
      Method::CompileGoSource {
        import_path: format!("mvdan.cc/sh/v3/cmd/shfmt@v{version}"),
      },
    ]
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("shfmt formats shell programs") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output("--version", log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn download_url(version: &Version, platform: Platform) -> String {
  let os = match platform.os {
    Os::Linux => "linux",
    Os::MacOS => "darwin",
    Os::Windows => "windows",
  };
  let cpu = match platform.cpu {
    Cpu::Arm64 => "arm64",
    Cpu::Intel64 => "amd64",
  };
  let ext = match platform.os {
    Os::Linux | Os::MacOS => "",
    Os::Windows => ".exe",
  };
  format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/shfmt_v{version}_{os}_{cpu}{ext}")
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"^v(\d+\.\d+\.\d+)$")
}

#[cfg(test)]
mod tests {
  use crate::configuration::Version;
  use crate::platform::{Cpu, Os, Platform};
  use crate::UserError;

  #[test]
  fn artifact_url() {
    let platform = Platform { os: Os::MacOS, cpu: Cpu::Arm64 };
    let have = super::download_url(&Version::from("3.7.0"), platform);
    let want = "https://github.com/mvdan/sh/releases/download/v3.7.0/shfmt_v3.7.0_darwin_arm64";
    assert_eq!(have, want);
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("v3.7.0"), Ok("3.7.0"));
    assert_eq!(super::extract_version("3.7.0"), Err(UserError::RegexDoesntMatch));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
