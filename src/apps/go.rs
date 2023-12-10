use std::path::PathBuf;

use super::App;
use crate::hosting::github;
use crate::install::archive::{self, Args};
use crate::platform::{Cpu, Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};
use const_format::formatcp;

pub struct Go {}

const ORG: &str = "golang";
const REPO: &str = "go";

impl App for Go {
    fn name(&self) -> &'static str {
        "Go"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "go.exe",
            Os::Linux | Os::MacOS => "go",
        }
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://go.dev")
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        archive::install(Args {
            artifact_url: download_url(version, platform),
            folder_on_disk: yard.app_folder(self.name(), version),
            trim: "",
            output,
        })?;
        Ok(Some(Executable(PathBuf::from(format!("bin/{}", self.executable_filename(platform))))))
    }

    fn latest_version(&self, output: &dyn Output) -> Result<String> {
        github::latest(ORG, REPO, output)
    }

    fn versions(&self, amount: u8, output: &dyn Output) -> Result<Vec<String>> {
        github::versions("antham", "ghokin", amount, output)
    }
}

fn download_url(version: &str, platform: Platform) -> String {
    format!(
        "https://go.dev/dl/go{version}.{os}-{cpu}.{ext}",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu),
        ext = ext_text(platform.os)
    )
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "linux",
        Os::MacOS => "darwin",
        Os::Windows => "windows",
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "arm64",
        Cpu::Intel64 => "amd64",
    }
}

fn ext_text(os: Os) -> &'static str {
    match os {
        Os::Windows => "zip",
        Os::Linux | Os::MacOS => "tar.gz",
    }
}

#[cfg(test)]
mod tests {
    mod download_url {
        use crate::platform::{Cpu, Os, Platform};

        #[test]
        fn macos_intel64() {
            let platform = Platform {
                os: Os::MacOS,
                cpu: Cpu::Intel64,
            };
            let have = super::super::download_url("3.4.1", platform);
            let want = "https://github.com/antham/ghokin/releases/download/v3.4.1/ghokin_3.4.1_darwin_amd64.tar.gz";
            assert_eq!(have, want);
        }

        #[test]
        fn windows_intel64() {
            let platform = Platform {
                os: Os::Windows,
                cpu: Cpu::Intel64,
            };
            let have = super::super::download_url("3.4.1", platform);
            let want = "https://github.com/antham/ghokin/releases/download/v3.4.1/ghokin_3.4.1_windows_amd64.tar.gz";
            assert_eq!(have, want);
        }
    }
}