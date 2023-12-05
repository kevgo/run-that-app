use super::App;
use crate::install::{ArtifactType, DownloadPrecompiledBinary, InstallationMethod};
use crate::platform::{Cpu, Os, Platform};
use crate::yard::Yard;

pub struct GolangCiLint {}

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
        "https://github.com/golangci/golangci-lint"
    }

    fn install(&self, version: &str, platform: Platform, yard: &Yard) -> Result<Option<Executable>> {
        todo!()
    }
    fn installation_methods(&self, version: &str, platform: Platform, yard: &Yard) -> Vec<Box<dyn InstallationMethod>> {
        vec![
            Box::new(DownloadPrecompiledBinary {
                name: self.name(),
                url: download_url(version, platform),
                artifact_type: ArtifactType::Archive {
                    file_to_extract: format!(
                        "golangci-lint-{version}-{os}-{cpu}/{executable}",
                        os = os_text(platform.os),
                        cpu = cpu_text(platform.cpu),
                        executable = self.executable_filename(platform)
                    ),
                },
                file_on_disk: yard.app_file_path(self.name(), version, self.executable_filename(platform)),
            }),
            // install from source not recommended, see https://golangci-lint.run/usage/install/#install-from-source
        ]
    }
}

fn download_url(version: &str, platform: Platform) -> String {
    format!(
        "https://github.com/golangci/golangci-lint/releases/download/v{version}/golangci-lint-{version}-{os}-{cpu}.{ext}",
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
