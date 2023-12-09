use super::App;
use crate::hosting::github;
use crate::install::{download_executable, ArtifactType, DownloadArgs};
use crate::platform::{Cpu, Os, Platform};
use crate::yard::{Executable, Yard};
use crate::{Output, Result};
use const_format::formatcp;

pub struct GolangCiLint {}

const ORG: &str = "golangci";
const REPO: &str = "golangci-lint";

impl App for GolangCiLint {
    fn name(&self) -> &'static str {
        "golangci-lint"
    }

    fn executable_filename(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "golangci-lint.exe",
            Os::Linux | Os::MacOS => "golangci-lint",
        }
    }

    fn homepage(&self) -> &'static str {
        formatcp!("https://github.com/{ORG}/{REPO}")
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard, output: &dyn Output) -> Result<Option<Executable>> {
        download_executable(&DownloadArgs {
            app_name: self.name(),
            artifact_url: download_url(version, platform),
            artifact_type: ArtifactType::PackagedExecutable {
                file_to_extract: format!(
                    "golangci-lint-{version}-{os}-{cpu}/{executable}",
                    os = os_text(platform.os),
                    cpu = cpu_text(platform.cpu),
                    executable = self.executable_filename(platform)
                ),
            },
            file_on_disk: yard.app_file_path(self.name(), version, self.executable_filename(platform)),
            output,
        })
        // install from source not recommended, see https://golangci-lint.run/usage/install/#install-from-source
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
        "https://github.com/{ORG}/{REPO}/releases/download/v{version}/golangci-lint-{version}-{os}-{cpu}.{ext}",
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
        Os::Linux | Os::MacOS => "tar.gz",
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
        let have = super::download_url("1.55.2", platform);
        let want = "https://github.com/golangci/golangci-lint/releases/download/v1.55.2/golangci-lint-1.55.2-darwin-arm64.tar.gz";
        assert_eq!(have, want);
    }
}
