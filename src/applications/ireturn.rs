use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::run::Executable;
use crate::Log;
use const_format::formatcp;

pub struct Ireturn {}

const ORG: &str = "butuzov";
const REPO: &str = "ireturn";

impl App for Ireturn {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("ireturn")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_methods(&self, version: &Version, platform: Platform) -> Vec<installation::Method> {
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "darwin",
      Os::Windows => "windows",
    };
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "x86_64",
    };
    let ext = match platform.os {
      Os::Linux | Os::MacOS => "tar.gz",
      Os::Windows => "zip",
    };
    vec![
      Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/ireturn_{os}_{cpu}.{ext}"),
        path_in_archive: self.executable_filename(platform),
      },
      Method::CompileGoSource {
        import_path: format!("github.com/{ORG}/{REPO}/cmd/ireturn@v{version}"),
      },
    ]
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("ireturn: Accept Interfaces, Return Concrete Types") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::ireturn::Ireturn;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (Ireturn {}).run_methods(
        &Version::from("0.3.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = vec![
        Method::DownloadArchive {
          url: S("https://github.com/butuzov/ireturn/releases/download/v0.3.0/ireturn_linux_x86_64.tar.gz"),
          path_in_archive: S("ireturn"),
        },
        Method::CompileGoSource {
          import_path: S("github.com/butuzov/ireturn/cmd/ireturn@v0.3.0"),
        },
      ];
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Ireturn {}).run_methods(
        &Version::from("0.3.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = vec![
        Method::DownloadArchive {
          url: S("https://github.com/butuzov/ireturn/releases/download/v0.3.0/ireturn_windows_x86_64.zip"),
          path_in_archive: S("ireturn.exe"),
        },
        Method::CompileGoSource {
          import_path: S("github.com/butuzov/ireturn/cmd/ireturn@v0.3.0"),
        },
      ];
      assert_eq!(have, want);
    }
  }
}
