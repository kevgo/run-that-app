use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::{Cpu, Os, Platform};
use crate::run::Executable;
use crate::{prelude::*, run};
use crate::{regexp, Log};
use const_format::formatcp;

pub struct Tikibase {}

const ORG: &str = "kevgo";
const REPO: &str = "tikibase";

impl App for Tikibase {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("tikibase")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> run::Method {
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "intel64",
    };
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "macos",
      Os::Windows => "windows",
    };
    let ext = match platform.os {
      Os::Linux | Os::MacOS => "tar.gz",
      Os::Windows => "zip",
    };
    run::Method::ThisApp {
      install_methods: vec![Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/tikibase_{os}_{cpu}.{ext}"),
        paths_in_archive: self.executable_filename(platform),
      }],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Linter for Markdown-based knowledge databases") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output("--version", log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }

  fn clone(&self) -> Box<dyn App> {
    Box::new(Tikibase {})
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"tikibase (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod install_methods {
    use crate::applications::tikibase::Tikibase;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    #[cfg(unix)]
    fn linux_arm() {
      use crate::run;

      let have = (Tikibase {}).run_method(
        &Version::from("0.6.2"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = run::Method::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: S("https://github.com/kevgo/tikibase/releases/download/v0.6.2/tikibase_macos_arm64.tar.gz"),
          paths_in_archive: S("tikibase"),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn windows_intel() {
      let have = (Tikibase {}).install_methods(
        &Version::from("0.6.2"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = vec![Method::DownloadArchive {
        url: S("https://github.com/kevgo/tikibase/releases/download/v0.6.2/tikibase_windows_intel64.zip"),
        path_in_archive: S("tikibase.exe"),
      }];
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("tikibase 0.6.2"), Ok("0.6.2"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
