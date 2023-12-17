use super::App;
use crate::hosting::github;
use crate::install::archive::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};

pub struct Npm {}

const ORG: &str = "nodejs";
const REPO: &str = "node";

impl App for Npm {
    fn name(&self) -> &'static str {
        "npm"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "npm.exe",
            Os::Linux | Os::MacOS => "npm",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://nodejs.org"
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        archive::install(InstallArgs {
            artifact_url: download_url(version, platform),
            filepath_on_disk: yard.app_file_path(self.name(), version, self.executable_filename(platform)),
            executable_path_in_archive: &format!("bin/{}", self.executable_filename(platform)),
            output,
        })
    }

    fn latest_version(&self, output: &dyn Output) -> Result<String> {
        github::latest(ORG, REPO, output)
    }

    fn versions(&self, amount: u8, output: &dyn Output) -> Result<Vec<String>> {
        github::versions(ORG, REPO, amount, output)
    }
}

fn download_url(version: &str, platform: Platform) -> String {
    format!(
        "https://nodejs.org/dist/v{version}/node-v{version}-{os}-{cpu}.{ext}",
        os = os_text(platform.os),
        cpu = cpu_text(platform.cpu),
        ext = ext_text(platform.os)
    )
}

fn os_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "linux",
        Os::MacOS => "darwin",
        Os::Windows => "win",
    }
}

fn cpu_text(cpu: Cpu) -> &'static str {
    match cpu {
        Cpu::Arm64 => "arm64",
        Cpu::Intel64 => "x64",
    }
}

fn ext_text(os: Os) -> &'static str {
    match os {
        Os::Linux => "tar.xz",
        Os::MacOS => "tar.gz",
        Os::Windows => "zip",
    }
}

#[cfg(test)]
mod tests {
    use crate::platform::{Cpu, Os, Platform};

    #[test]
    fn download_url() {
        let platform = Platform {
            os: Os::MacOS,
            cpu: Cpu::Arm64,
        };
        let have = super::download_url("20.10.0", platform);
        let want = "https://nodejs.org/dist/v20.10.0/node-v20.10.0-darwin-arm64.tar.gz";
        assert_eq!(have, want);
    }
}