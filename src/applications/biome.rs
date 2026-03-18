use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::Version;
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::hosting::github_releases;
use crate::installation::Method;
use crate::platform::{Cpu, Os, Platform};
use crate::{Log, strings};
use const_format::formatcp;

#[derive(Clone)]
pub(crate) struct Biome {}

const ORG: &str = "biomejs";
const REPO: &str = "biome";
const TAG_PREFIX: &str = "@biomejs/biome@";

impl AppDefinition for Biome {
  fn name(&self) -> ApplicationName {
    "biome".into()
  }

  fn homepage(&self) -> &'static str {
    formatcp!("https://github.com/{ORG}/{REPO}")
  }

  fn run_method(&self, version: &Version, platform: Platform) -> RunMethod {
    let cpu = match platform.cpu {
      Cpu::Arm64 => "arm64",
      Cpu::Intel64 => "x64",
    };
    let os = match platform.os {
      Os::Linux => "linux",
      Os::MacOS => "darwin",
      Os::Windows => "win32",
    };
    let ext = match platform.os {
      Os::Windows => ".exe",
      Os::Linux | Os::MacOS => "",
    };
    RunMethod::ThisApp {
      install_methods: vec![Method::DownloadExecutable {
        url: format!(
          "https://github.com/{ORG}/{REPO}/releases/download/{tag}{version}/biome-{os}-{cpu}{ext}",
          tag = urlencoding::encode(TAG_PREFIX)
        )
        .into(),
      }],
    }
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    github_releases::versions(ORG, REPO, amount, TAG_PREFIX, log)
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    github_releases::latest(ORG, REPO, TAG_PREFIX, log)
  }

  fn analyze_executable(&self, executable: &Executable, log: Log) -> Result<AnalyzeResult> {
    let output = executable.run_output(&["-h"], log)?;
    if !output.contains("Biome official CLI.") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    match extract_version(&executable.run_output(&["--version"], log)?) {
      Ok(version) => Ok(AnalyzeResult::IdentifiedWithVersion(version.into())),
      Err(_) => Ok(AnalyzeResult::IdentifiedButUnknownVersion),
    }
  }
}

fn extract_version(output: &str) -> Result<&str> {
  strings::first_capture(output, r"Version: (\d+\.\d+\.\d+)")
}

#[cfg(test)]
mod tests {

  mod install_methods {
    use crate::applications::AppDefinition;
    use crate::applications::biome::Biome;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::installation::{BinFolder, Method};
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn linux_arm() {
      let have = (Biome {}).run_method(
        &Version::from("2.4.8"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/biomejs/biome/releases/download/%40biomejs%2Fbiome%402.4.8/biome-linux-arm64".into(),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn linux_x86() {
      let have = (Biome {}).run_method(
        &Version::from("2.4.8"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/biomejs/biome/releases/download/%40biomejs%2Fbiome%402.4.8/biome-linux-x64".into(),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_arm() {
      let have = (Biome {}).run_method(
        &Version::from("2.4.8"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/biomejs/biome/releases/download/%40biomejs%2Fbiome%402.4.8/biome-darwin-arm64".into(),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn macos_x86() {
      let have = (Biome {}).run_method(
        &Version::from("2.4.8"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/biomejs/biome/releases/download/%40biomejs%2Fbiome%402.4.8/biome-darwin-x64".into(),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_arm() {
      let have = (Biome {}).run_method(
        &Version::from("2.4.8"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/biomejs/biome/releases/download/%40biomejs%2Fbiome%402.4.8/biome-win32-arm64.exe".into(),
        }],
      };
      assert_eq!(have, want);
    }

    #[test]
    fn windows_x86() {
      let have = (Biome {}).run_method(
        &Version::from("2.4.8"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::ThisApp {
        install_methods: vec![Method::DownloadExecutable {
          url: "https://github.com/biomejs/biome/releases/download/%40biomejs%2Fbiome%402.4.8/biome-win32-x64.exe".into(),
        }],
      };
      assert_eq!(have, want);
    }
  }
}
