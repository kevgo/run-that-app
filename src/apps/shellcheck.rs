use super::App;
use crate::hosting::github_releases;
use crate::install::packaged_executable::{self, InstallArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};

pub struct ShellCheck {}

const ORG: &str = "koalaman";
const REPO: &str = "shellcheck";

impl App for ShellCheck {
    fn name(&self) -> &'static str {
        "shellcheck"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Linux | Os::MacOS => "shellcheck",
            Os::Windows => "shellcheck.exe",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://www.shellcheck.net"
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        packaged_executable::install(InstallArgs {
            app_name: self.name(),
            artifact_url: download_url(version, platform),
            file_to_extract: &format!("shellcheck-v{version}/{executable}", executable = self.executable_filename(platform)),
            filepath_on_disk: yard.app_file_path(self.name(), version, self.executable_filename(platform)),
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
        "https://github.com/{ORG}/{REPO}/releases/download/v{version}/shellcheck-v{version}.{os}.{cpu}.{ext}",
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
        Cpu::Arm64 => "aarch64",
        Cpu::Intel64 => "x86_64",
    }
}

fn ext_text(os: Os) -> &'static str {
    match os {
        Os::Linux | Os::MacOS => "tar.xz",
        Os::Windows => "zip",
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
        let have = super::download_url("0.9.0", platform);
        let want = "https://github.com/koalaman/shellcheck/releases/download/v0.9.0/shellcheck-v0.9.0.linux.x86_64.tar.xz";
        assert_eq!(have, want);
    }
}
