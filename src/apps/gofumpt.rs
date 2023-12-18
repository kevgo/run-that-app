use super::App;
use crate::hosting::github;
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::install::executable::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};
use const_format::formatcp;

pub struct Gofumpt {}

const ORG: &str = "mvdan";
const REPO: &str = "gofumpt";

impl App for Gofumpt {
    fn name(&self) -> &'static str {
        "gofumpt"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "gofumpt.exe",
            Os::Linux | Os::MacOS => "gofumpt",
        }
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn latest_version(&self, output: &dyn Output) -> Result<String> {
        github::latest(ORG, REPO, output)
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        if let Some(executable) = executable::install(InstallArgs {
            artifact_url: download_url(version, platform),
            filepath_on_disk: yard.app_file_path(self.name(), version, self.executable_filename(platform)),
            output,
        })? {
            return Ok(Some(executable));
        }
        compile_go(CompileArgs {
            import_path: format!("mvdan.cc/gofumpt@{version}"),
            target_folder: yard.app_folder(self.name(), version),
            executable_filename: self.executable_filename(platform),
            output,
        })
    }

    fn load(&self, version: &str, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(self.name(), version, self.executable_filename(platform))
    }

    fn versions(&self, amount: u8, output: &dyn Output) -> Result<Vec<String>> {
        github::versions(ORG, REPO, amount, output)
    }
}

fn download_url(version: &str, platform: Platform) -> String {
    format!(
        "https://github.com/{ORG}/{REPO}/releases/download/v{version}/gofumpt_v{version}_{os}_{cpu}{ext}",
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
        Os::Windows => ".exe",
        Os::Linux | Os::MacOS => "",
    }
}

#[cfg(test)]
mod tests {
    mod download_url {
        use crate::platform::{Cpu, Os, Platform};

        #[test]
        fn macos_arm64() {
            let platform = Platform {
                os: Os::MacOS,
                cpu: Cpu::Arm64,
            };
            let have = super::super::download_url("0.5.0", platform);
            let want = "https://github.com/mvdan/gofumpt/releases/download/v0.5.0/gofumpt_v0.5.0_darwin_arm64";
            assert_eq!(have, want);
        }

        #[test]
        fn windows_intel64() {
            let platform = Platform {
                os: Os::Windows,
                cpu: Cpu::Intel64,
            };
            let have = super::super::download_url("0.5.0", platform);
            let want = "https://github.com/mvdan/gofumpt/releases/download/v0.5.0/gofumpt_v0.5.0_windows_amd64.exe";
            assert_eq!(have, want);
        }
    }
}
