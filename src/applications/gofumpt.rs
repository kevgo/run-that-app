use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{regexp, Log};
use const_format::formatcp;

pub struct Gofumpt {}

const ORG: &str = "mvdan";
const REPO: &str = "gofumpt";

impl App for Gofumpt {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("gofumpt")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn install_methods(&self) -> Vec<installation::Method> {
    vec![Method::DownloadExecutable(self), Method::CompileGoSource(self)]
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("display diffs instead of rewriting files") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output("--version", log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

impl installation::DownloadExecutable for Gofumpt {
  fn download_url(&self, version: &Version, platform: Platform) -> String {
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
      Os::Windows => ".exe",
      Os::Linux | Os::MacOS => "",
    };
    format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/gofumpt_v{version}_{os}_{cpu}{ext}")
  }
}

impl installation::CompileGoSource for Gofumpt {
  fn import_path(&self, version: &Version) -> String {
    format!("mvdan.cc/gofumpt@v{version}")
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"v(\d+\.\d+\.\d+) \(go")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod artifact_url {
    use crate::configuration::Version;
    use crate::installation::DownloadExecutable;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn macos_arm64() {
      let gofumpt = super::super::Gofumpt {};
      let platform = Platform { os: Os::MacOS, cpu: Cpu::Arm64 };
      let have = gofumpt.download_url(&Version::from("0.5.0"), platform);
      let want = "https://github.com/mvdan/gofumpt/releases/download/v0.5.0/gofumpt_v0.5.0_darwin_arm64";
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel64() {
      let gofumpt = super::super::Gofumpt {};
      let platform = Platform { os: Os::Windows, cpu: Cpu::Intel64 };
      let have = gofumpt.download_url(&Version::from("0.5.0"), platform);
      let want = "https://github.com/mvdan/gofumpt/releases/download/v0.5.0/gofumpt_v0.5.0_windows_amd64.exe";
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("v0.6.0 (go1.21.6)"), Ok("0.6.0"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
