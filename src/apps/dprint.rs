use super::App;
use crate::hosting::github_releases;
use crate::install::compile_rust::{compile_rust, CompileArgs};
use crate::install::packaged_executable::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::subshell::Executable;
use crate::yard::Yard;
use crate::{Output, Result};

pub struct Dprint {}

const ORG: &str = "dprint";
const REPO: &str = "dprint";

impl App for Dprint {
    fn name(&self) -> &'static str {
        "dprint"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "dprint",
            Os::Windows => "dprint.exe",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://dprint.dev"
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        if let Some(executable) = packaged_executable::install(InstallArgs {
            app_name: self.name(),
            artifact_url: download_url(version, platform),
            file_to_extract: self.executable_filename(platform),
            filepath_on_disk: yard.app_folder(self.name(), version).join(self.executable_filename(platform)),
            output,
        })? {
            return Ok(Some(executable));
        }
        compile_rust(CompileArgs {
            crate_name: "dprint",
            target_folder: yard.app_folder(self.name(), version),
            executable_filename: self.executable_filename(platform),
            output,
        })
    }

    fn latest_installable_version(&self, output: &dyn Output) -> Result<String> {
        github_releases::latest(ORG, REPO, output)
    }

    fn load(&self, version: &str, platform: Platform, yard: &Yard) -> Option<Executable> {
        yard.load_app(self.name(), version, self.executable_filename(platform))
    }

    fn installable_versions(&self, amount: usize, output: &dyn Output) -> Result<Vec<String>> {
        github_releases::versions(ORG, REPO, amount, output)
    }
}

fn download_url(version: &str, platform: Platform) -> String {
    format!(
        "https://github.com/{ORG}/{REPO}/releases/download/{version}/dprint-{cpu}-{os}.zip",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu)
    )
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "unknown-linux-gnu",
        Os::MacOS => "apple-darwin",
        Os::Windows => "pc-windows-msvc",
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "aarch64",
        Cpu::Intel64 => "x86_64",
    }
}

#[cfg(test)]
mod tests {
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn mac_arm() {
        let platform = Platform {
            os: Os::MacOS,
            cpu: Cpu::Arm64,
        };
        let have = super::download_url("0.43.0", platform);
        let want = "https://github.com/dprint/dprint/releases/download/0.43.0/dprint-aarch64-apple-darwin.zip";
        assert_eq!(have, want);
    }

    #[test]
    fn linux_arm() {
        let platform = Platform {
            os: Os::Linux,
            cpu: Cpu::Arm64,
        };
        let have = super::download_url("0.43.1", platform);
        let want = "https://github.com/dprint/dprint/releases/download/0.43.1/dprint-aarch64-unknown-linux-gnu.zip";
        assert_eq!(have, want);
    }
}
