use super::{AnalyzeResult, App};
use crate::configuration::{ApplicationName, Version};
use crate::hosting::github_releases;
use crate::installation::{self, Method};
use crate::platform::{Cpu, Os, Platform};
use crate::prelude::*;
use crate::subshell::Executable;
use crate::Log;
use const_format::formatcp;

pub struct Ghokin {}

const ORG: &str = "antham";
const REPO: &str = "ghokin";

impl App for Ghokin {
  fn name(&self) -> ApplicationName {
    ApplicationName::from("ghokin")
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn install_methods(&self, version: &Version, platform: Platform) -> Vec<installation::Method> {
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "amd64",
    };
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "darwin",
      Os::Windows => "windows",
    };
    vec![
      Method::DownloadArchive {
        url: format!("https://github.com/{ORG}/{REPO}/releases/download/v{version}/ghokin_{version}_{os}_{cpu}.tar.gz"),
        path_in_archive: self.executable_filename(platform),
      },
      Method::CompileGoSource {
        import_path: format!("github.com/{ORG}/{REPO}/v3@v{version}"),
      },
    ]
  }
  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions("antham", "ghokin", amount, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output("-h", log)?;
    if !output.contains("Clean and/or apply transformation on gherkin files") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // as of 3.4.0 ghokin's "version" command prints nothing
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::ghokin::Ghokin;
    use crate::applications::App;
    use crate::configuration::Version;
    use crate::installation::Method;
    use crate::platform::{Cpu, Os, Platform};
    use big_s::S;

    #[test]
    fn linux_arm() {
      let have = (Ghokin {}).install_methods(
        &Version::from("3.4.1"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = vec![
        Method::DownloadArchive {
          url: S("https://github.com/antham/ghokin/releases/download/v3.4.1/ghokin_3.4.1_darwin_amd64.tar.gz"),
          path_in_archive: S("ghokin"),
        },
        Method::CompileGoSource {
          import_path: S("github.com/antham/ghokin/v3@v3.4.1"),
        },
      ];
      assert_eq!(have, want);
    }

    #[test]
    fn windows_intel() {
      let have = (Ghokin {}).install_methods(
        &Version::from("3.7.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = vec![
        Method::DownloadArchive {
          url: S("https://github.com/antham/ghokin/releases/download/v3.7.0/ghokin_3.7.0_windows_amd64.tar.gz"),
          path_in_archive: S("ghokin.exe"),
        },
        Method::CompileGoSource {
          import_path: S("github.com/antham/ghokin/v3@v3.7.0"),
        },
      ];
      assert_eq!(have, want);
    }
  }
}
