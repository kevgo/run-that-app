use super::App;
use crate::detect::{Cpu, Os, Platform};
use crate::install::DownloadPrecompiledBinary;

pub struct GolangCiLint {}

impl App for GolangCiLint {
    fn name(&self) -> &'static str {
        "golangci-lint"
    }

    fn executable(&self, platform: Platform) -> &'static str {
        match platform.os {
            Os::Windows => "golangci-lint.exe",
            Os::Linux | Os::MacOS => "golangci-lint",
        }
    }

    fn homepage(&self) -> &'static str {
        "https://github.com/golangci/golangci-lint"
    }

    fn installation_methods(
        &self,
        version: &str,
        platform: Platform,
        yard: &crate::yard::Yard,
    ) -> Vec<Box<dyn crate::install::InstallationMethod>> {
        vec![
            Box::new(DownloadPrecompiledBinary {
                url: format!("https://github.com/golangci/golangci-lint/releases/download/v{version}/golangci-lint-{version}-{os}-{cpu}.{ext}", os = os_text(platform.os), cpu = cpu_text(platform.cpu), ext = ext_text(platform.os)),
                file_in_archive: None,
                file_on_disk: yard.app_file_path(self.name(), version, self.executable(platform)),
            }),
            // install from source not recommended, see https://golangci-lint.run/usage/install/#install-from-source
        ]
    }
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
