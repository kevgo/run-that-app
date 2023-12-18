use super::App;
use crate::hosting::github;
use crate::install::compile_go::{compile_go, CompileArgs};
use crate::install::executable::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};
use const_format::formatcp;

pub struct Depth {}

const ORG: &str = "KyleBanks";
const REPO: &str = "depth";

impl App for Depth {
    fn name(&self) -> &'static str {
        "depth"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "depth.exe",
            Os::Linux | Os::MacOS => "depth",
        }
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        if let Some(executable) = executable::install(InstallArgs {
            app_name: self.name(),
            artifact_url: download_url(version, platform),
            filepath_on_disk: yard.app_file_path(self.name(), version, self.executable_filename(platform)),
            output,
        })? {
            return Ok(Some(executable));
        }
        compile_go(CompileArgs {
            import_path: format!("github.com/{ORG}/{REPO}/cmd/depth@v{version}"),
            target_folder: yard.app_folder(self.name(), version),
            executable_filename: self.executable_filename(platform),
            output,
        })
    }

    fn latest_version(&self, output: &dyn Output) -> Result<String> {
        github::latest(ORG, REPO, output)
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
        "https://github.com/{ORG}/{REPO}/releases/download/v{version}/depth_{version}_{os}_{cpu}",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu)
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
        Cpu::Arm64 => "arm",
        Cpu::Intel64 => "amd64",
    }
}

#[cfg(test)]
mod tests {
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn download_url() {
        let platform = Platform {
            os: Os::Linux,
            cpu: Cpu::Intel64,
        };
        let have = super::download_url("1.2.1", platform);
        let want = "https://github.com/KyleBanks/depth/releases/download/v1.2.1/depth_1.2.1_linux_amd64";
        assert_eq!(have, want);
    }
}
