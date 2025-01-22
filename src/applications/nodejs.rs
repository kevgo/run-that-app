use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::run::ExecutablePath;
use crate::{regexp, run, Log};
use std::path;

pub struct NodeJS {}

pub const ORG: &str = "nodejs";
pub const REPO: &str = "node";

impl App for NodeJS {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("node")
  }

  fn homepage(&self) -> &'static str {
    "https://nodejs.org"
  }

  fn run_method(&self, version: &Version, platform: Platform) -> run::Method {
    let os = os_text(platform.os);
    let cpu = cpu_text(platform.cpu);
    let ext = ext_text(platform.os);
    let sep = path::MAIN_SEPARATOR;
    run::Method::ThisApp {
      install_methods: vec![Method::DownloadArchive {
        url: format!("https://nodejs.org/dist/v{version}/node-v{version}-{os}-{cpu}.{ext}",),
        bin_folders: BinFolder::RootOrSubfolders {
          options: vec![format!("node-v{version}-{os}-{cpu}"), format!("node-v{version}-{os}-{cpu}{sep}bin")],
        },
      }],
    }
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &ExecutablePath, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Documentation can be found at https://nodejs.org") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output("--version", log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }

  fn clone(&self) -> Box<dyn App> {
    Box::new(Self {})
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"v(\d+\.\d+\.\d+)")
}

pub fn cpu_text(cpu: Cpu) -> &'static str {
  match cpu {
    Cpu::Arm64 => "arm64",
    Cpu::Intel64 => "x64",
  }
}

fn ext_text(os: Os) -> &'static str {
  match os {
    Os::Linux => "tar.xz",
    Os::MacOS => "tar.gz",
    Os::Windows => "zip",
  }
}

pub fn os_text(os: Os) -> &'static str {
  match os {
    Os::Linux => "linux",
    Os::MacOS => "darwin",
    Os::Windows => "win",
  }
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod install_methods {
    use crate::applications::nodejs::NodeJS;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};
    use crate::run;
    use big_s::S;

    #[test]
    #[cfg(unix)]
    fn linux_arm() {
      use crate::installation::BinFolder;

      let have = (NodeJS {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = run::Method::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: S("https://nodejs.org/dist/v20.10.0/node-v20.10.0-darwin-arm64.tar.gz"),
          bin_folders: BinFolder::RootOrSubfolders {
            options: vec![S("node-v20.10.0-darwin-arm64"), S("node-v20.10.0-darwin-arm64/bin")],
          },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn windows_intel() {
      let have = (NodeJS {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = run::Method::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: S("https://nodejs.org/dist/v20.10.0/node-v20.10.0-win-x64.zip"),
          bin_folders: BinFolder::RootOrSubfolders {
            options: vec![S("node-v20.10.0-win-x64"), S("node-v20.10.0-win-x64\\bin")],
          },
        }],
      };
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("v10.2.4"), Ok("10.2.4"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
