use super::{AnalyzeResult, AppDefinition};
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::{BinFolder, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, regexp};
use const_format::formatcp;

#[derive(Clone)]
pub(crate) struct ActionLint {}

const ORG: &str = "rhysd";
const REPO: &str = "actionlint";

impl AppDefinition for ActionLint {
  fn name(&self) -> &'static str {
    "actionlint"
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://{ORG}.github.io/{REPO}")
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
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
    RunMethod::ThisApp {
      install_methods: vec![
        Method::DownloadArchive {
          url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/actionlint_{version}_{os}_{cpu}.{ext}"),
          bin_folder: BinFolder::Root,
        },
        Method::CompileGoSource {
          import_path: format!("github.com/{ORG}/{REPO}/cmd/actionlint@v{version}"),
        },
      ],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("actionlint is a linter for GitHub Actions workflow files") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    let output = executable.run_output(&["--version"], log)?;
    match extract_version(&output) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::NotIdentified { output }),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"(\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod run_method {
    use crate::applications::AppDefinition;
    use crate::applications::actionlint::ActionLint;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (ActionLint {}).run_method(
        &Version::from("1.6.26"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: S("https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_linux_arm64.tar.gz"),
            bin_folder: BinFolder::Root,
          },
          Method::CompileGoSource {
            import_path: S("github.com/rhysd/actionlint/cmd/actionlint@v1.6.26"),
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (ActionLint {}).run_method(
        &Version::from("1.6.26"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadArchive {
            url: S("https://github.com/rhysd/actionlint/releases/download/v1.6.26/actionlint_1.6.26_windows_amd64.zip"),
            bin_folder: BinFolder::Root,
          },
          Method::CompileGoSource {
            import_path: S("github.com/rhysd/actionlint/cmd/actionlint@v1.6.26"),
          },
        ],
      };
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("actionlint 1.6.27"), Ok("1.6.27"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
