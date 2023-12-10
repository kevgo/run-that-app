use super::App;
use crate::hosting::github;
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::install::packaged_executable::{self, Args};
use crate::platform::{Cpu, Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};
use const_format::formatcp;

pub struct Ghokin {}

const ORG: &str = "antham";
const REPO: &str = "ghokin";

impl App for Ghokin {
    fn name(&self) -> &'static str {
        "ghokin"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "ghokin.exe",
            Os::Linux | Os::MacOS => "ghokin",
        }
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        if let Some(executable) = packaged_executable::install(&Args {
            artifact_url: download_url(version, platform),
            file_to_extract: self.executable_filename(platform),
            filepath_on_disk: yard.app_folder(self.name(), version),
            output,
        })? {
            return Ok(Some(executable));
        }
        compile_go(&CompileArgs {
            import_path: format!("github.com/{ORG}/{REPO}/v3@v{version}"),
            target_folder: yard.app_folder(self.name(), version),
            executable_filename: self.executable_filename(platform),
            output,
        })
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
        "https://github.com/{ORG}/{REPO}/releases/download/v{version}/ghokin_{version}_{os}_{cpu}.tar.gz",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu),
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
