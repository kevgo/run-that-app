use super::{AnalyzeResult, App};
use crate::config::{AppName, Version};
use crate::hosting::github_releases;
use crate::install::Method;
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::{install, Log};

const ORG: &str = "dominikh";
const REPO: &str = "go-tools";

pub struct StaticCheck {}

impl App for StaticCheck {
  fn name(&self) -> AppName {
    AppName::from("staticcheck")
  }

  fn homepage(&self) -> &'static str {
    "https://staticcheck.dev"
  }

  fn install_methods(&self) -> Vec<install::Method> {
    vec![Method::DownloadArchive(self), Method::CompileGoSource(self)]
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Usage: staticcheck [flags] [packages]") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

impl install::DownloadArchive for StaticCheck {
  fn archive_url(&self, version: &Version, platform: Platform) -> String {
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "darwin",
      Os::Windows => "windows",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "amd64",
    };
    format!("https://github.com/{ORG}/{REPO}/releases/download/{version}/staticcheck_{os}_{cpu}.tar.gz")
  }

  fn executable_path_in_archive(&self, _version: &Version, platform: Platform) -> String {
    format!("staticcheck/{}", self.executable_filename(platform))
  }
}

impl install::CompileGoSource for StaticCheck {
  fn import_path(&self, version: &Version) -> String {
    format!("honnef.co/go/tools/cmd/staticcheck@{version}")
  }
}
