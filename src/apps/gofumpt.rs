use super::App;
use crate::install::{ArtifactType, CompileFromGoSource, DownloadPrecompiledBinary, InstallationMethod};
use crate::platform::{Cpu, Os, Platform};
use crate::yard::Yard;

pub struct Gofumpt {}

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
        "https://github.com/mvdan/gofumpt"
    }

    fn installation_methods(&self, version: &str, platform: Platform, yard: &Yard) -> Vec<Box<dyn InstallationMethod>> {
        vec![
            Box::new(DownloadPrecompiledBinary {
                name: self.name(),
                url: download_url(version, platform),
                artifact_type: ArtifactType::Executable,
                file_on_disk: yard.app_file_path(self.name(), version, self.executable_filename(platform)),
            }),
            Box::new(CompileFromGoSource {
                import_path: format!("mvdan.cc/gofumpt@{version}"),
                target_folder: yard.app_folder(self.name(), version),
                executable_filename: self.executable_filename(platform),
            }),
        ]
    }
}

fn download_url(version: &str, platform: Platform) -> String {
    format!(
        "https://github.com/mvdan/gofumpt/releases/download/v{version}/gofumpt_v{version}_{os}_{cpu}{ext}",
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
