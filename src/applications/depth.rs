use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::run::Executable;
use crate::{run, Log};
use const_format::formatcp;

pub struct Depth {}

const ORG: &str = "KyleBanks";
const REPO: &str = "depth";

impl App for Depth {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("depth")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> run::Method {
    let cpu = match platform.cpu {
      Cpu::Arm64 => "aarch64", // the "arm" binaries don't run on Apple Silicon
      Cpu::Intel64 => "amd64",
    };
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "darwin",
      Os::Windows => "windows",
    };
    let ext = match platform.os {
      Os::Windows => ".exe",
      Os::Linux | Os::MacOS => "",
    };
    run::Method::ThisApp {
      install_methods: vec![
        Method::DownloadExecutable {
          url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/depth_{version}_{os}_{cpu}{ext}"),
        },
        Method::CompileGoSource {
          import_path: format!("github.com/{ORG}/{REPO}/cmd/depth@v{version}"),
        },
      ],
    }
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("resolves dependencies of internal (stdlib) packages.") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // as of 1.2.1 depth doesn't display the version of the installed executable
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }

  fn clone(&self) -> Box<dyn App> {
    Box::new(Self {})
  }
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use super::super::Depth;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use crate::run;
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (Depth {}).run_method(
        &Version::from("1.2.1"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = run::Method::ThisApp {
        install_methods: vec![
          Method::DownloadExecutable {
            url: S("https://github.com/KyleBanks/depth/releases/download/v1.2.1/depth_1.2.1_linux_aarch64"),
          },
          Method::CompileGoSource {
            import_path: S("github.com/KyleBanks/depth/cmd/depth@v1.2.1"),
          },
        ],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Depth {}).run_method(
        &Version::from("1.2.1"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = run::Method::ThisApp {
        install_methods: vec![
          Method::DownloadExecutable {
            url: S("https://github.com/KyleBanks/depth/releases/download/v1.2.1/depth_1.2.1_windows_amd64.exe"),
          },
          Method::CompileGoSource {
            import_path: S("github.com/KyleBanks/depth/cmd/depth@v1.2.1"),
          },
        ],
      };
      assert_eq!(have, want);
    }
  }
}
