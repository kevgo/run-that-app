use super::App;
use crate::hosting::github;
use crate::install::{download_executable, ArtifactType, DownloadArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};

pub struct ShellCheck {}

impl App for ShellCheck {
    fn name(&self) -> &'static str {
        "shellcheck"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "shellcheck.exe",
            Os::Linux | Os::MacOS => "shellcheck",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://www.shellcheck.net"
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        download_executable(&DownloadArgs {
            app_name: self.name(),
            artifact_url: download_url(version, platform),
            artifact_type: ArtifactType::Archive {
                file_to_extract: format!("shellcheck-v{version}/{executable}", executable = self.executable_filename(platform)),
            },
            file_on_disk: yard.app_file_path(self.name(), version, self.executable_filename(platform)),
            output,
        })
    }

    fn versions(&self, amount: u8, output: &dyn Output) -> Result<Vec<String>> {
        github::versions("koalaman", "shellcheck", amount, output)
    }
}

fn download_url(version: &str, platform: Platform) -> String {
    format!(
        "https://github.com/koalaman/shellcheck/releases/download/v{version}/shellcheck-v{version}.{os}.{cpu}.{ext}",
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
