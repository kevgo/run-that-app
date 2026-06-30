use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::{TagFormat, Version};
use crate::error::{Result, UserError};
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_tags;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, filesystem, strings};
use std::path::MAIN_SEPARATOR;

#[derive(Clone)]
pub struct Go {}

const ORG: &str = "golang";
const REPO: &str = "go";

impl AppDefinition for Go {
  fn name(&self) -> ApplicationName {
    "go".into()
  }

  fn homepage(&self) -> &'static str {
    "https://go.dev"
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
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
      Os::Linux | Os::MacOS => "tar.gz",
      Os::Windows => "zip",
    };
    let tag = self.tag_format().format_version(version);
    RunMethod::ThisApp {
      install_methods: vec![Method::DownloadArchive {
        url: format!("https://go.dev/dl/{tag}.{os}-{cpu}.{ext}").into(),
        bin_folder: BinFolder::Subfolder {
          path: format!("go{MAIN_SEPARATOR}bin").into(),
        },
      }],
    }
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    let versions = self.installable_versions(1, log)?;
    let Some(version) = versions.into_iter().next() else {
      return Err(UserError::NoVersionsFound { app: self.name().to_string() });
    };
    Ok(version)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    let tags = github_tags::all(ORG, REPO, 400, &self.tag_format(), log)?;
    let mut go_tags: Vec<Version> = tags
      .into_iter()
      .filter(|tag| !tag.as_str().contains("rc"))
      .filter(|tag| !tag.as_str().contains("beta"))
      .filter(|tag| !tag.as_str().starts_with("release"))
      .filter(|tag| !tag.as_str().starts_with("weekly"))
      .collect();
    go_tags.sort_unstable_by(|a, b| human_sort::compare(b.as_str(), a.as_str()));
    if go_tags.len() > amount {
      go_tags.resize(amount, Version::from(""));
    }
    Ok(go_tags.into_iter().collect())
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    if let Ok(version) = strings::first_version(&executable.run_output(&["version"], log)?) {
      return Ok(AnalyzeResult::IdentifiedWithVersion(version.into()));
    }
    let output = executable.run_output(&["-h"], log)?;
    if output.contains("Go is a tool for managing Go source code") {
      Ok(AnalyzeResult::IdentifiedButUnknownVersion)
    } else {
      Ok(AnalyzeResult::NotIdentified { output })
    }
  }

  fn allowed_versions(&self) -> Result<semver::VersionReq> {
    let Some(go_mod_content) = filesystem::read_file("go.mod")? else {
      return Ok(semver::VersionReq::STAR);
    };
    let Ok(go_version_req) = parse_go_mod(&go_mod_content) else {
      return Ok(semver::VersionReq::STAR);
    };
    let version_req = semver::VersionReq::parse(go_version_req).map_err(|err| UserError::CannotParseSemverRange {
      expression: go_version_req.to_string(),
      reason: err.to_string(),
    })?;
    Ok(version_req)
  }

  fn tag_format(&self) -> TagFormat {
    TagFormat::Prefix("go")
  }
}

fn parse_go_mod(text: &str) -> Result<&str> {
  strings::first_capture(text, r"(?m)^go\s+(\d+\.\d+)\s*$")
}

#[cfg(test)]
mod tests {

  mod run_method {
    use crate::applications::AppDefinition;
    use crate::applications::go::Go;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    #[cfg(not(windows))]
    fn linux_arm() {
      let have = (Go {}).run_method(
        &Version::from("1.21.5"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://go.dev/dl/go1.21.5.linux-arm64.tar.gz".into(),
          bin_folder: BinFolder::Subfolder { path: "go/bin".into() },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(not(windows))]
    fn linux_intel() {
      let have = (Go {}).run_method(
        &Version::from("1.21.5"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://go.dev/dl/go1.21.5.linux-amd64.tar.gz".into(),
          bin_folder: BinFolder::Subfolder { path: "go/bin".into() },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(not(windows))]
    fn macos_arm() {
      let have = (Go {}).run_method(
        &Version::from("1.21.5"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://go.dev/dl/go1.21.5.darwin-arm64.tar.gz".into(),
          bin_folder: BinFolder::Subfolder { path: "go/bin".into() },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(not(windows))]
    fn macos_intel() {
      let have = (Go {}).run_method(
        &Version::from("1.21.5"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://go.dev/dl/go1.21.5.darwin-amd64.tar.gz".into(),
          bin_folder: BinFolder::Subfolder { path: "go/bin".into() },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn windows_arm() {
      let have = (Go {}).run_method(
        &Version::from("1.21.5"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://go.dev/dl/go1.21.5.windows-arm64.zip".into(),
          bin_folder: BinFolder::Subfolder { path: r"go\bin".into() },
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn windows_intel() {
      let have = (Go {}).run_method(
        &Version::from("1.21.5"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadArchive {
          url: "https://go.dev/dl/go1.21.5.windows-amd64.zip".into(),
          bin_folder: BinFolder::Subfolder { path: r"go\bin".into() },
        }],
      };
      assert_eq!(have, want);
    }
  }

  mod parse_go_mod {
    use crate::UserError;
    use crate::applications::go::parse_go_mod;

    #[test]
    fn with_version() {
      let go_mod = "
module github.com/git-town/git-town/v12

go 1.21

require (
	code.gitea.io/sdk/gitea v0.17.1
	github.com/BurntSushi/toml v1.3.2
	github.com/acarl005/stripansi v0.0.0-20180116102854-5a71ef0e047d
	github.com/charmbracelet/bubbles v0.18.0
	github.com/charmbracelet/bubbletea v0.25.0
)";
      assert_eq!(parse_go_mod(go_mod), Ok("1.21"));
    }

    #[test]
    fn without_version() {
      let go_mod = "
module github.com/git-town/git-town/v12

require (
	code.gitea.io/sdk/gitea v0.17.1
	github.com/BurntSushi/toml v1.3.2
	github.com/acarl005/stripansi v0.0.0-20180116102854-5a71ef0e047d
	github.com/charmbracelet/bubbles v0.18.0
	github.com/charmbracelet/bubbletea v0.25.0
)";
      assert_eq!(parse_go_mod(go_mod), Err(UserError::RegexDoesntMatch));
    }

    #[test]
    fn unrelated_file() {
      let go_mod = "content from file coincidentally also named go.mod";
      assert_eq!(parse_go_mod(go_mod), Err(UserError::RegexDoesntMatch));
    }
  }
}
