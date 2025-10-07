use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, regexp};
use const_format::formatcp;

#[derive(Clone)]
pub(crate) struct Gofumpt {}

const ORG: &str = "mvdan";
const REPO: &str = "gofumpt";

impl AppDefinition for Gofumpt {
  fn name(&self) -> ApplicationName {
    "gofumpt".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
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
      Os::Windows => ".exe",
      Os::Linux | Os::MacOS => "",
    };
    RunMethod::ThisApp {
      install_methods: vec![
        Method::DownloadExecutable {
          url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/gofumpt_v{version}_{os}_{cpu}{ext}").into(),
        },
        Method::CompileGoSource {
          import_path: format!("mvdan.cc/gofumpt@v{version}"),
        },
      ],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("display diffs instead of rewriting files") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  regexp::first_capture(output, r"v(\d+\.\d+\.\d+) \(go")
}

#[cfg(test)]
mod tests {
  use crate::UserError;

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::gofumpt::Gofumpt;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (Gofumpt {}).run_method(
        &Version::from("0.5.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadExecutable {
            url: "https://github.com/mvdan/gofumpt/releases/download/v0.5.0/gofumpt_v0.5.0_darwin_arm64".into(),
          },
          Method::CompileGoSource {
            import_path: S("mvdan.cc/gofumpt@v0.5.0"),
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Gofumpt {}).run_method(
        &Version::from("0.5.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![
          Method::DownloadExecutable {
            url: "https://github.com/mvdan/gofumpt/releases/download/v0.5.0/gofumpt_v0.5.0_windows_amd64.exe".into(),
          },
          Method::CompileGoSource {
            import_path: S("mvdan.cc/gofumpt@v0.5.0"),
          },
        ],
      };
      assert_eq!(have, want);
    }
  }

  #[test]
  fn extract_version() {
    assert_eq!(super::extract_version("v0.6.0 (go1.21.6)"), Ok("0.6.0"));
    assert_eq!(super::extract_version("other"), Err(UserError::RegexDoesntMatch));
  }
}
