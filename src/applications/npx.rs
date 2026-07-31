use super::nodejs::NodeJS;
use super::{AnalyzeResult, AppDefinition, ApplicationName};
use crate::configuration::{TagFormat, Version};
use crate::error::Result;
use crate::executables::{Executable, RunMethod};
use crate::platform::{Os, Platform};
use crate::{Log, subshell};

#[derive(Clone)]
pub struct Npx {}

impl AppDefinition for Npx {
  fn name(&self) -> ApplicationName {
    "npx".into()
  }

  fn homepage(&self) -> &'static str {
    "https://www.npmjs.com"
  }

  fn run_method(&self, _version: &Version, platform: Platform) -> RunMethod {
    RunMethod::OtherAppShellScript {
      carrier: Box::new(NodeJS {}),
      script_name: match platform.os {
        Os::Linux | Os::MacOS => "npx",
        Os::Windows => "npx.cmd",
      },
    }
  }

  fn latest_installable_version(&self, log: Log) -> Result<Version> {
    app_to_install().latest_installable_version(log)
  }

  fn installable_versions(&self, amount: usize, log: Log) -> Result<Vec<Version>> {
    app_to_install().installable_versions(amount, log)
  }

  fn analyze_executable(&self, executable: &Executable) -> Result<AnalyzeResult> {
    let output = subshell::capture_output(executable, &["-h"])?;
    if !output.contains("Run a command from a local or remote npm package") {
      return Ok(AnalyzeResult::NotIdentified { output });
    }
    // Npx is versioned together with NodeJS. The actual version of npm is therefore not relevant here.
    Ok(AnalyzeResult::IdentifiedButUnknownVersion)
  }

  fn tag_format(&self) -> TagFormat {
    app_to_install().tag_format()
  }
}

fn app_to_install() -> NodeJS {
  NodeJS {}
}

#[cfg(test)]
mod tests {

  mod run_method {
    use crate::applications::AppDefinition;
    use crate::applications::nodejs::NodeJS;
    use crate::applications::npx::Npx;
    use crate::configuration::Version;
    use crate::executables::RunMethod;
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    #[cfg(not(windows))]
    fn linux_arm() {
      let have = (Npx {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::OtherAppShellScript {
        carrier: Box::new(NodeJS {}),
        script_name: "npx",
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(not(windows))]
    fn linux_intel() {
      let have = (Npx {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::Linux,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::OtherAppShellScript {
        carrier: Box::new(NodeJS {}),
        script_name: "npx",
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(not(windows))]
    fn macos_arm() {
      let have = (Npx {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::OtherAppShellScript {
        carrier: Box::new(NodeJS {}),
        script_name: "npx",
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(not(windows))]
    fn macos_intel() {
      let have = (Npx {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::MacOS,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::OtherAppShellScript {
        carrier: Box::new(NodeJS {}),
        script_name: "npx",
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn windows_arm() {
      let have = (Npx {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Arm64,
        },
      );
      let want = RunMethod::OtherAppShellScript {
        carrier: Box::new(NodeJS {}),
        script_name: "npx.cmd",
      };
      assert_eq!(have, want);
    }

    #[test]
    #[cfg(windows)]
    fn windows_intel() {
      let have = (Npx {}).run_method(
        &Version::from("20.10.0"),
        Platform {
          os: Os::Windows,
          cpu: Cpu::Intel64,
        },
      );
      let want = RunMethod::OtherAppShellScript {
        carrier: Box::new(NodeJS {}),
        script_name: "npx.cmd",
      };
      assert_eq!(have, want);
    }
  }
}
